use async_trait::async_trait;
use ruvector_core::types::{DbOptions,HnswConfig};
use ruvector_core::{DistanceMetric,SearchQuery,VectorDB,VectorEntry};
use rvf_crypto::{WitnessEntry,create_witness_chain,shake256_256,verify_witness_chain};
use serde_json::Value;
use std::{collections::HashMap,path::Path};
use toolrecall_application::{EvidencePort,WitnessPort};
use toolrecall_domain::SelectionReceipt;

pub struct RuvectorEvidence{db:VectorDB}
impl RuvectorEvidence{pub fn open(path:impl AsRef<Path>)->Result<Self,String>{let p=path.as_ref();let storage=if p.is_dir(){p.join("outcomes.redb")}else{p.to_path_buf()};let o=DbOptions{dimensions:8,distance_metric:DistanceMetric::Cosine,storage_path:storage.to_string_lossy().into_owned(),hnsw_config:Some(HnswConfig::default()),quantization:None};VectorDB::new(o).map(|db|Self{db}).map_err(|e|e.to_string())}}
#[async_trait]impl EvidencePort for RuvectorEvidence{async fn append_and_search(&self,r:&SelectionReceipt)->Result<Vec<String>,String>{let v=embedding(r);let prior=self.db.search(SearchQuery{vector:v.clone(),k:3,filter:None,ef_search:Some(32)}).map_err(|e|e.to_string())?.into_iter().map(|x|x.id).collect();let mut m=HashMap::<String,Value>::new();m.insert("authority".into(),Value::String("none".into()));m.insert("query_id".into(),Value::String(r.query_id.clone()));let id=self.db.insert(VectorEntry{id:Some(format!("{}-{}",r.query_id,&r.catalog_sha256[..12])),vector:v,metadata:Some(m)}).map_err(|e|e.to_string())?;self.db.get(&id).map_err(|e|e.to_string())?.ok_or("RuVector read-back missing")?;Ok(prior)}}
pub struct RvfWitness;
impl WitnessPort for RvfWitness{fn seal(&self,payloads:&[Vec<u8>])->Result<String,String>{let entries:Vec<_>=payloads.iter().enumerate().map(|(i,p)|WitnessEntry{prev_hash:[0;32],action_hash:shake256_256(p),timestamp_ns:i as u64,witness_type:if i==0{1}else{2}}).collect();let c=create_witness_chain(&entries);verify_witness_chain(&c).map_err(|e|e.to_string())?;Ok(hex::encode(shake256_256(&c)))}}
fn embedding(r:&SelectionReceipt)->Vec<f32>{vec![r.selected.len() as f32/100.0,r.missing_sources.len() as f32/10.0,r.missing_tools.len() as f32/10.0,r.query_id.len() as f32/100.0,r.catalog_sha256.as_bytes()[0] as f32/255.0,1.0,0.0,1.0]}

#[cfg(test)]mod tests{use super::*;use tempfile::tempdir;use toolrecall_application::EvidencePort;#[tokio::test]async fn real_ruvector_and_rvf_round_trip(){let d=tempdir().unwrap();let e=RuvectorEvidence::open(d.path()).unwrap();let r=SelectionReceipt{query_id:"q".into(),selected:vec![],missing_sources:vec![],missing_tools:vec![],catalog_sha256:"a".repeat(64),authority:"none".into()};assert!(e.append_and_search(&r).await.unwrap().is_empty());assert_eq!(e.append_and_search(&r).await.unwrap().len(),1);assert_eq!(RvfWitness.seal(&[b"a".to_vec(),b"b".to_vec()]).unwrap().len(),64);}}
