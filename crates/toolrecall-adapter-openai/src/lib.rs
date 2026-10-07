use async_trait::async_trait;
use std::path::{Path, PathBuf};
use tokio::{
    process::Command,
    time::{Duration, timeout},
};
use toolrecall_application::CatalogPort;
use toolrecall_domain::ToolDescriptor;

pub struct OpenAiCatalogSidecar {
    python: PathBuf,
    script: PathBuf,
    timeout: Duration,
}
impl OpenAiCatalogSidecar {
    pub fn new(python: impl AsRef<Path>, script: impl AsRef<Path>) -> Self {
        Self {
            python: python.as_ref().into(),
            script: script.as_ref().into(),
            timeout: Duration::from_secs(10),
        }
    }
}
#[async_trait]
impl CatalogPort for OpenAiCatalogSidecar {
    async fn load(&self) -> Result<Vec<ToolDescriptor>, String> {
        let out = timeout(
            self.timeout,
            Command::new(&self.python).arg(&self.script).output(),
        )
        .await
        .map_err(|_| "OpenAI sidecar timeout".to_string())?
        .map_err(|e| e.to_string())?;
        if !out.status.success() {
            return Err(String::from_utf8_lossy(&out.stderr).into_owned());
        }
        serde_json::from_slice(&out.stdout).map_err(|e| e.to_string())
    }
}
