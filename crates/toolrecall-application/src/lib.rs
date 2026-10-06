use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use toolrecall_domain::{QueryCase, RecallPolicy, SelectionReceipt, ToolDescriptor, select};

#[derive(Debug, Error)] pub enum AppError { #[error("catalog: {0}")] Catalog(String), #[error("memory: {0}")] Memory(String), #[error("witness: {0}")] Witness(String), #[error(transparent)] Domain(#[from] toolrecall_domain::DomainError) }
#[async_trait] pub trait CatalogPort: Send + Sync { async fn load(&self)->Result<Vec<ToolDescriptor>,String>; }
#[async_trait] pub trait EvidencePort: Send + Sync { async fn append_and_search(&self, receipt:&SelectionReceipt)->Result<Vec<String>,String>; }
pub trait WitnessPort: Send + Sync { fn seal(&self, payloads:&[Vec<u8>])->Result<String,String>; }

#[derive(Debug, Serialize, Deserialize)] pub struct WorkflowReceipt { pub selection:SelectionReceipt, pub similar_outcomes:Vec<String>, pub witness_root:String, pub authority:String }
pub async fn run<C: CatalogPort, E: EvidencePort, W: WitnessPort>(catalog:&C,evidence:&E,witness:&W,q:&QueryCase,p:&RecallPolicy)->Result<WorkflowReceipt,AppError>{
    let tools=catalog.load().await.map_err(AppError::Catalog)?; let selection=select(q,&tools,p)?;
    let similar_outcomes=evidence.append_and_search(&selection).await.map_err(AppError::Memory)?;
    let witness_root=witness.seal(&[serde_json::to_vec(q).expect("serializable"),serde_json::to_vec(&selection).expect("serializable")]).map_err(AppError::Witness)?;
    Ok(WorkflowReceipt{selection,similar_outcomes,witness_root,authority:"none".into()})
}
