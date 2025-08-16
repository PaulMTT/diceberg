use mistralrs::{Model, RequestLike, Response};
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use crate::term::duplex::DuplexSink;

#[derive(Debug, Default, Clone, Copy)]
pub enum CancelCtl {
    #[default]
    Idle,
    AbortCurrent,
    Shutdown,
}

pub enum ChatEvent {
    Response(Response),
    Cancelled,
    Complete,
}

impl fmt::Debug for ChatEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChatEvent::Response(_) => f.write_str("ChatEvent::Response(<Response>)"),
            ChatEvent::Cancelled => f.write_str("ChatEvent::Cancelled"),
            ChatEvent::Complete => f.write_str("ChatEvent::Complete"),
        }
    }
}

pub struct MistralDuplexSink<S>
where
    S: DuplexSink<ToSource = ChatEvent>,
    S::FromSource: RequestLike,
{
    sink: S,
    model: Arc<Model>,
}

impl<S> MistralDuplexSink<S>
where
    S: DuplexSink<ToSource = ChatEvent, Cancel = CancelCtl>,
    S::FromSource: RequestLike,
{
    pub fn new(sink: S, model: Arc<Model>) -> Self {
        Self { sink, model }
    }

    pub async fn run(mut self) {
        let mut cancel_rx = self.sink.cancel_rx().clone();

        loop {
            let req = tokio::select! {
                _ = cancel_rx.changed() => {
                    match *cancel_rx.borrow_and_update() {
                        CancelCtl::Shutdown => break,
                        CancelCtl::AbortCurrent | CancelCtl::Idle => continue,
                    }
                }
                maybe_req = self.sink.recv_from_source() => {
                    match maybe_req {
                        Some(r) => r,
                        None => break,
                    }
                }
            };

            let _ = *cancel_rx.borrow_and_update();

            match self.model.stream_chat_request(req).await {
                Err(e) => {
                    let _ = self
                        .sink
                        .send_to_source(ChatEvent::Response(Response::InternalError(e.into())))
                        .await;
                }
                Ok(mut stream) => loop {
                    tokio::select! {
                        _ = cancel_rx.changed() => {
                            let ctl = *cancel_rx.borrow_and_update();
                            match ctl {
                                CancelCtl::Shutdown => {
                                    terminate_engine_now();
                                    while let Some(_ignored) = stream.next().await {}
                                    reset_engine_termination();
                                    let _ = self.sink.send_to_source(ChatEvent::Cancelled).await;
                                    return;
                                }
                                CancelCtl::AbortCurrent => {
                                    terminate_engine_now();
                                    while let Some(_ignored) = stream.next().await {}
                                    reset_engine_termination();
                                    let _ = self.sink.send_to_source(ChatEvent::Cancelled).await;
                                    break;
                                }
                                CancelCtl::Idle => {  }
                            }
                        }
                        item = stream.next() => {
                            match item {
                                Some(resp) => {

                                    let _ = self.sink.send_to_source(ChatEvent::Response(resp)).await;
                                }
                                None => {

                                    reset_engine_termination();
                                    let _ = self.sink.send_to_source(ChatEvent::Complete).await;
                                    break;
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

impl<S> MistralDuplexSink<S>
where
    S: DuplexSink<ToSource = ChatEvent, Cancel = CancelCtl> + Send + 'static,
    S::FromSource: RequestLike + Send + 'static,
    S::Cancel: Send + Sync + 'static,
{
    pub fn spawn(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move { self.run().await })
    }
}

fn terminate_engine_now() {
    mistralrs::get_engine_terminate_flag().store(true, Ordering::SeqCst);
    mistralrs::TERMINATE_ALL_NEXT_STEP.store(true, Ordering::SeqCst);
}

fn reset_engine_termination() {
    mistralrs::reset_engine_terminate_flag();
    mistralrs::TERMINATE_ALL_NEXT_STEP.store(false, Ordering::SeqCst);
}
