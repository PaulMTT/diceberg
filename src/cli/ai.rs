use crate::term::duplex::Duplex;
use crate::term::llm_chat_sink::{CancelCtl, ChatEvent, MistralDuplexSink};
use crate::term::llm_chat_ui_source::MistralDuplexSourceUi;
use mistralrs::{
    IsqType, ModelDType,
    PagedAttentionMetaBuilder, RequestBuilder, TextModelBuilder,
};
use std::sync::Arc;
use anyhow::Result;

#[cfg(feature = "mcp")]
use mistralrs::{McpServerConfig, McpClientConfig, McpServerSource};
pub async fn handle_ai() -> Result<()> {
    #[cfg(feature = "mcp")]
    let mcp = McpClientConfig {
        servers: vec![McpServerConfig {
            name: "The data and insights cloud integration (DICI) model context protocol (MCP) server.".into(),
            source: McpServerSource::Process {
                command: "dici".into(),
                args: vec!["serve".into(), "mcp".into()],
                work_dir: None,
                env: None,
            },
            tool_prefix: None,
            ..Default::default()
        }],
        auto_register_tools: true,
        tool_timeout_secs: Some(30),
        max_concurrent_calls: Some(4),
        ..Default::default()
    };
    let model = TextModelBuilder::new("Qwen/Qwen3-4B")
        .with_dtype(ModelDType::Auto)
        .with_isq(IsqType::AFQ8)
        .with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?;
    #[cfg(feature = "mcp")]
    let model = model.with_mcp_client(mcp);
    let model = model.build().await?;
    let (source, sink) = Duplex::unbounded::<RequestBuilder, ChatEvent, CancelCtl>();
    let _worker = MistralDuplexSink::new(sink, Arc::new(model)).spawn();
    let mut ui = MistralDuplexSourceUi::new(source);
    ui.run()
}
