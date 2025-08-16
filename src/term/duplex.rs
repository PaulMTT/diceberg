use core::future::Future;
use tokio::sync::mpsc::error as mpsc_err;
use tokio::sync::{mpsc, watch};

#[derive(Debug)]
pub enum Tx<T> {
    Bounded(mpsc::Sender<T>),
    Unbounded(mpsc::UnboundedSender<T>),
}

impl<T> Tx<T> {
    pub fn try_send(&self, v: T) -> Result<(), mpsc_err::TrySendError<T>> {
        match self {
            Tx::Bounded(tx) => tx.try_send(v),
            Tx::Unbounded(tx) => tx.send(v).map_err(|e| mpsc_err::TrySendError::Closed(e.0)),
        }
    }

    pub async fn send(&self, v: T) -> Result<(), mpsc::error::SendError<T>> {
        match self {
            Tx::Bounded(tx) => tx.send(v).await,
            Tx::Unbounded(tx) => tx.send(v),
        }
    }
}

#[derive(Debug)]
pub enum Rx<T> {
    Bounded(mpsc::Receiver<T>),
    Unbounded(mpsc::UnboundedReceiver<T>),
}

impl<T> Rx<T> {
    pub fn try_recv(&mut self) -> Result<T, mpsc_err::TryRecvError> {
        match self {
            Rx::Bounded(rx) => rx.try_recv(),
            Rx::Unbounded(rx) => rx.try_recv(),
        }
    }

    pub async fn recv(&mut self) -> Option<T> {
        match self {
            Rx::Bounded(rx) => rx.recv().await,
            Rx::Unbounded(rx) => rx.recv().await,
        }
    }
}

#[derive(Debug)]
pub struct SourceHandle<ToSink, FromSink, Cancel = u64> {
    to_sink_tx: Tx<ToSink>,
    from_sink_rx: Rx<FromSink>,
    cancel_tx: watch::Sender<Cancel>,
}

#[derive(Debug)]
pub struct SinkHandle<ToSink, FromSink, Cancel = u64> {
    from_source_rx: Rx<ToSink>,
    to_source_tx: Tx<FromSink>,
    cancel_rx: watch::Receiver<Cancel>,
}

#[derive(Debug)]
pub struct Duplex;

impl Duplex {
    pub fn unbounded<ToSink, FromSink, Cancel>() -> (
        SourceHandle<ToSink, FromSink, Cancel>,
        SinkHandle<ToSink, FromSink, Cancel>,
    )
    where
        ToSink: Send + 'static,
        FromSink: Send + 'static,
        Cancel: Default + Send + Sync + 'static,
    {
        let (to_sink_tx, from_source_rx) = mpsc::unbounded_channel::<ToSink>();
        let (to_source_tx, from_sink_rx) = mpsc::unbounded_channel::<FromSink>();
        let (cancel_tx, cancel_rx) = watch::channel::<Cancel>(Cancel::default());

        (
            SourceHandle {
                to_sink_tx: Tx::Unbounded(to_sink_tx),
                from_sink_rx: Rx::Unbounded(from_sink_rx),
                cancel_tx,
            },
            SinkHandle {
                from_source_rx: Rx::Unbounded(from_source_rx),
                to_source_tx: Tx::Unbounded(to_source_tx),
                cancel_rx,
            },
        )
    }

    pub fn bounded<ToSink, FromSink, Cancel>(
        to_sink_cap: usize,
        to_source_cap: usize,
    ) -> (
        SourceHandle<ToSink, FromSink, Cancel>,
        SinkHandle<ToSink, FromSink, Cancel>,
    )
    where
        ToSink: Send + 'static,
        FromSink: Send + 'static,
        Cancel: Default + Send + Sync + 'static,
    {
        let (to_sink_tx, from_source_rx) = mpsc::channel::<ToSink>(to_sink_cap);
        let (to_source_tx, from_sink_rx) = mpsc::channel::<FromSink>(to_source_cap);
        let (cancel_tx, cancel_rx) = watch::channel::<Cancel>(Cancel::default());

        (
            SourceHandle {
                to_sink_tx: Tx::Bounded(to_sink_tx),
                from_sink_rx: Rx::Bounded(from_sink_rx),
                cancel_tx,
            },
            SinkHandle {
                from_source_rx: Rx::Bounded(from_source_rx),
                to_source_tx: Tx::Bounded(to_source_tx),
                cancel_rx,
            },
        )
    }
}

pub trait DuplexSource {
    type ToSink;
    type FromSink;
    type Cancel;

    type ToSinkTx;
    type FromSinkRx;

    fn to_sink_tx(&self) -> &Self::ToSinkTx;
    fn from_sink_rx(&mut self) -> &mut Self::FromSinkRx;

    fn cancel_tx(&self) -> &watch::Sender<Self::Cancel>;

    fn try_send_to_sink(
        &self,
        msg: Self::ToSink,
    ) -> Result<(), mpsc_err::TrySendError<Self::ToSink>>;
    fn try_recv_from_sink(&mut self) -> Result<Self::FromSink, mpsc_err::TryRecvError>;

    fn send_to_sink(
        &mut self,
        msg: Self::ToSink,
    ) -> impl Future<Output = Result<(), mpsc::error::SendError<Self::ToSink>>> + Send;

    fn recv_from_sink(&mut self) -> impl Future<Output = Option<Self::FromSink>> + Send;
}

pub trait DuplexSink {
    type FromSource;
    type ToSource;
    type Cancel;

    type FromSourceRx;
    type ToSourceTx;

    fn from_source_rx(&mut self) -> &mut Self::FromSourceRx;
    fn to_source_tx(&self) -> &Self::ToSourceTx;

    fn cancel_rx(&self) -> &watch::Receiver<Self::Cancel>;

    fn try_recv_from_source(&mut self) -> Result<Self::FromSource, mpsc_err::TryRecvError>;
    fn try_send_to_source(
        &self,
        msg: Self::ToSource,
    ) -> Result<(), mpsc_err::TrySendError<Self::ToSource>>;

    fn send_to_source(
        &mut self,
        msg: Self::ToSource,
    ) -> impl Future<Output = Result<(), mpsc::error::SendError<Self::ToSource>>> + Send;

    fn recv_from_source(&mut self) -> impl Future<Output = Option<Self::FromSource>> + Send;
}

impl<ToSink, FromSink, Cancel> DuplexSource for SourceHandle<ToSink, FromSink, Cancel>
where
    ToSink: Send + 'static,
    FromSink: Send + 'static,
    Cancel: Default + Send + Sync + 'static,
{
    type ToSink = ToSink;
    type FromSink = FromSink;
    type Cancel = Cancel;

    type ToSinkTx = Tx<ToSink>;
    type FromSinkRx = Rx<FromSink>;

    fn to_sink_tx(&self) -> &Self::ToSinkTx {
        &self.to_sink_tx
    }
    fn from_sink_rx(&mut self) -> &mut Self::FromSinkRx {
        &mut self.from_sink_rx
    }
    fn cancel_tx(&self) -> &watch::Sender<Self::Cancel> {
        &self.cancel_tx
    }

    fn try_send_to_sink(
        &self,
        msg: Self::ToSink,
    ) -> Result<(), mpsc_err::TrySendError<Self::ToSink>> {
        self.to_sink_tx.try_send(msg)
    }
    fn try_recv_from_sink(&mut self) -> Result<Self::FromSink, mpsc_err::TryRecvError> {
        self.from_sink_rx.try_recv()
    }

    fn send_to_sink(
        &mut self,
        msg: Self::ToSink,
    ) -> impl Future<Output = Result<(), mpsc::error::SendError<Self::ToSink>>> + Send {
        async move { self.to_sink_tx.send(msg).await }
    }

    fn recv_from_sink(&mut self) -> impl Future<Output = Option<Self::FromSink>> + Send {
        async move { self.from_sink_rx.recv().await }
    }
}

impl<ToSink, FromSink, Cancel> DuplexSink for SinkHandle<ToSink, FromSink, Cancel>
where
    ToSink: Send + 'static,
    FromSink: Send + 'static,
    Cancel: Default + Send + Sync + 'static,
{
    type FromSource = ToSink;
    type ToSource = FromSink;
    type Cancel = Cancel;

    type FromSourceRx = Rx<ToSink>;
    type ToSourceTx = Tx<FromSink>;

    fn from_source_rx(&mut self) -> &mut Self::FromSourceRx {
        &mut self.from_source_rx
    }
    fn to_source_tx(&self) -> &Self::ToSourceTx {
        &self.to_source_tx
    }
    fn cancel_rx(&self) -> &watch::Receiver<Self::Cancel> {
        &self.cancel_rx
    }

    fn try_recv_from_source(&mut self) -> Result<Self::FromSource, mpsc_err::TryRecvError> {
        self.from_source_rx.try_recv()
    }
    fn try_send_to_source(
        &self,
        msg: Self::ToSource,
    ) -> Result<(), mpsc_err::TrySendError<Self::ToSource>> {
        self.to_source_tx.try_send(msg)
    }

    fn send_to_source(
        &mut self,
        msg: Self::ToSource,
    ) -> impl Future<Output = Result<(), mpsc::error::SendError<Self::ToSource>>> + Send {
        async move { self.to_source_tx.send(msg).await }
    }

    fn recv_from_source(&mut self) -> impl Future<Output = Option<Self::FromSource>> + Send {
        async move { self.from_source_rx.recv().await }
    }
}

impl<'a, T> DuplexSource for &'a mut T
where
    T: DuplexSource + 'a,
{
    type ToSink = T::ToSink;
    type FromSink = T::FromSink;
    type Cancel = T::Cancel;

    type ToSinkTx = T::ToSinkTx;
    type FromSinkRx = T::FromSinkRx;

    fn to_sink_tx(&self) -> &Self::ToSinkTx {
        <T as DuplexSource>::to_sink_tx(&**self)
    }
    fn from_sink_rx(&mut self) -> &mut Self::FromSinkRx {
        <T as DuplexSource>::from_sink_rx(&mut **self)
    }
    fn cancel_tx(&self) -> &watch::Sender<Self::Cancel> {
        <T as DuplexSource>::cancel_tx(&**self)
    }

    fn try_send_to_sink(
        &self,
        msg: Self::ToSink,
    ) -> Result<(), mpsc_err::TrySendError<Self::ToSink>> {
        <T as DuplexSource>::try_send_to_sink(&**self, msg)
    }
    fn try_recv_from_sink(&mut self) -> Result<Self::FromSink, mpsc_err::TryRecvError> {
        <T as DuplexSource>::try_recv_from_sink(&mut **self)
    }

    fn send_to_sink(
        &mut self,
        msg: Self::ToSink,
    ) -> impl Future<Output = Result<(), mpsc::error::SendError<Self::ToSink>>> + Send {
        <T as DuplexSource>::send_to_sink(&mut **self, msg)
    }

    fn recv_from_sink(&mut self) -> impl Future<Output = Option<Self::FromSink>> + Send {
        <T as DuplexSource>::recv_from_sink(&mut **self)
    }
}

impl<'a, T> DuplexSink for &'a mut T
where
    T: DuplexSink + 'a,
{
    type FromSource = T::FromSource;
    type ToSource = T::ToSource;
    type Cancel = T::Cancel;

    type FromSourceRx = T::FromSourceRx;
    type ToSourceTx = T::ToSourceTx;

    fn from_source_rx(&mut self) -> &mut Self::FromSourceRx {
        <T as DuplexSink>::from_source_rx(&mut **self)
    }
    fn to_source_tx(&self) -> &Self::ToSourceTx {
        <T as DuplexSink>::to_source_tx(&**self)
    }
    fn cancel_rx(&self) -> &watch::Receiver<Self::Cancel> {
        <T as DuplexSink>::cancel_rx(&**self)
    }

    fn try_recv_from_source(&mut self) -> Result<Self::FromSource, mpsc_err::TryRecvError> {
        <T as DuplexSink>::try_recv_from_source(&mut **self)
    }
    fn try_send_to_source(
        &self,
        msg: Self::ToSource,
    ) -> Result<(), mpsc_err::TrySendError<Self::ToSource>> {
        <T as DuplexSink>::try_send_to_source(&**self, msg)
    }

    fn send_to_source(
        &mut self,
        msg: Self::ToSource,
    ) -> impl Future<Output = Result<(), mpsc::error::SendError<Self::ToSource>>> + Send {
        <T as DuplexSink>::send_to_source(&mut **self, msg)
    }

    fn recv_from_source(&mut self) -> impl Future<Output = Option<Self::FromSource>> + Send {
        <T as DuplexSink>::recv_from_source(&mut **self)
    }
}
