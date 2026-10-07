use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use std::{collections::BTreeSet, path::PathBuf};
use toolrecall_adapter_mcp::validate_catalog;
use toolrecall_adapter_openai::OpenAiCatalogSidecar;
use toolrecall_adapter_ruvnet::{RuvectorEvidence, RvfWitness};
use toolrecall_application::{CatalogPort, run};
use toolrecall_domain::{QueryCase, RecallPolicy, Strategy};
#[derive(Parser)]
struct Cli {
    #[arg(long)]
    query: String,
    #[arg(long, default_value = "integrations/openai_catalog.py")]
    catalog_script: PathBuf,
    #[arg(long, default_value = "python3")]
    python: PathBuf,
    #[arg(long, default_value = ".toolrecall-memory")]
    memory: PathBuf,
    #[arg(long, value_enum, default_value = "source-guard")]
    strategy: ArgStrategy,
    #[arg(long, default_value_t = 2)]
    top_k: usize,
    #[arg(long, default_value_t = 4)]
    max_loaded: usize,
    #[arg(long)]
    required_source: Vec<String>,
    #[arg(long)]
    required_tool: Vec<String>,
}
#[derive(Clone, ValueEnum)]
enum ArgStrategy {
    Global,
    SourceGuard,
    Full,
}
#[tokio::main]
async fn main() -> Result<()> {
    let c = Cli::parse();
    std::fs::create_dir_all(&c.memory)?;
    let source = OpenAiCatalogSidecar::new(&c.python, &c.catalog_script);
    let cat = source.load().await.map_err(anyhow::Error::msg)?;
    validate_catalog(&cat)
        .map_err(anyhow::Error::msg)
        .context("MCP catalog validation")?;
    let q = QueryCase {
        id: "cli".into(),
        text: c.query,
        required_sources: c.required_source.into_iter().collect::<BTreeSet<_>>(),
        required_tools: c.required_tool.into_iter().collect::<BTreeSet<_>>(),
    };
    let strategy = match c.strategy {
        ArgStrategy::Global => Strategy::GlobalTopK,
        ArgStrategy::SourceGuard => Strategy::SourceGuard,
        ArgStrategy::Full => Strategy::FullCatalog,
    };
    let p = RecallPolicy {
        strategy,
        top_k: c.top_k,
        min_per_required_source: 1,
        max_loaded: c.max_loaded.max(c.top_k),
    };
    let e = RuvectorEvidence::open(&c.memory).map_err(anyhow::Error::msg)?;
    let receipt = run(&source, &e, &RvfWitness, &q, &p).await?;
    println!("{}", serde_json::to_string_pretty(&receipt)?);
    Ok(())
}
