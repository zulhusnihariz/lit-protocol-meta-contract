use marine_rs_sdk::marine;
use serde::Deserialize;

#[marine]
pub struct MetaContractResult {
    pub result: bool,
    pub metadatas: Vec<FinalMetadata>,
    pub error_string: String,
}

#[marine]
pub struct FinalMetadata {
    pub public_key: String,
    pub alias: String,
    pub content: String,
    pub loose: i64,
    pub version: String,
}

#[marine]
#[derive(Debug, Clone)]
pub struct Metadata {
    pub hash: String,
    pub token_key: String,
    pub data_key: String,
    pub meta_contract_id: String,
    pub token_id: String,
    pub alias: String,
    pub cid: String,
    pub public_key: String,
    pub version: String,
    pub loose: i64,
}

#[marine]
#[derive(Debug, Clone)]
pub struct Transaction {
    pub hash: String,
    pub method: String,
    pub meta_contract_id: String,
    pub data_key: String,
    pub token_key: String,
    pub data: String,
    pub public_key: String,
    pub alias: String,
    pub timestamp: u64,
    pub chain_id: String,
    pub token_address: String,
    pub token_id: String,
    pub version: String,
    pub status: i64,
    pub mcdata: String,
}

#[marine]
#[derive(Debug, Default, Clone)]
pub struct MetaContract {
    pub hash: String,
    pub token_key: String,
    pub meta_contract_id: String,
    pub public_key: String,
    pub cid: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct SerdeMetadata {
  pub loose: i64,
}

#[derive(Debug, Default, Deserialize)]
pub struct AuthSig {
  pub sig: String,
  pub derived_via: String,
  pub signed_message: String,
  pub address: String
}

impl AuthSig {
  pub fn is_any_empty_field(&self) -> bool {
    self.sig.is_empty() || self.derived_via.is_empty() || self.signed_message.is_empty() || self.address.is_empty()
  }

}

#[derive(Debug, Default, Deserialize)]
pub struct NousAiMetadata {
  pub auth_sig: AuthSig,
  pub encrypted_symmetric_key:String,
  pub encrypted_string: String
}

