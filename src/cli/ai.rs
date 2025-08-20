use crate::term::duplex::Duplex;
use crate::term::llm_chat_sink::{CancelCtl, ChatEvent, MistralDuplexSink};
use crate::term::llm_chat_ui_source::MistralDuplexSourceUi;
use anyhow::Result;
use mistralrs::{IsqType, ModelDType, PagedAttentionMetaBuilder, RequestBuilder, TextModelBuilder};
#[cfg(feature = "mcp")]
use mistralrs::{McpClientConfig, McpServerConfig, McpServerSource};
use std::sync::Arc;
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
        ..Default::default()
    };
    let model = TextModelBuilder::new("Qwen/Qwen3-4B").with_dtype(ModelDType::Auto);
    #[cfg(feature = "metal")]
    let model = model.with_isq(IsqType::AFQ8);
    #[cfg(not(feature = "metal"))]
    let model = model.with_isq(IsqType::Q8K);
    let model = model.with_paged_attn(|| PagedAttentionMetaBuilder::default().build())?;
    #[cfg(feature = "mcp")]
    let model = model.with_mcp_client(mcp);
    let model = model
        .with_max_num_seqs(3)
        .with_prefix_cache_n(Some(5))
        .build().await?;
    let (source, sink) = Duplex::unbounded::<RequestBuilder, ChatEvent, CancelCtl>();
    let _worker = MistralDuplexSink::new(sink, Arc::new(model)).spawn();
    let mut ui = MistralDuplexSourceUi::new(source);
    ui.run()
}
