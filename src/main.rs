#![allow(improper_ctypes)]

mod types;
mod data;
mod defaults;

use std::collections::HashMap;
use data::DataStructFork;
use defaults::DEFAULT_IPFS_MULTIADDR;
use defaults::DEFAULT_TIMEOUT_SEC;
use marine_rs_sdk::marine;
use marine_rs_sdk::module_manifest;
use marine_rs_sdk::MountedBinaryResult;
use marine_rs_sdk::WasmLoggerBuilder;
use types::MetaContract;
use types::Metadata;
use types::Transaction;
use types::{ FinalMetadata, MetaContractResult, LitProtocolMetadata };
use ethabi::{ decode, ParamType };

module_manifest!();

pub fn main() {
    WasmLoggerBuilder::new()
        .with_log_level(log::LevelFilter::Info)
        .build()
        .unwrap();
}

#[marine]
pub fn on_execute(
    contract: MetaContract,
    metadatas: Vec<Metadata>,
    transaction: Transaction,
) -> MetaContractResult {
    let mut finals: Vec<FinalMetadata> = vec![];

    
    let serde_metadata: Result<LitProtocolMetadata, serde_json::Error> = serde_json::from_str(&transaction.data.clone());

    match serde_metadata {
      Ok(metadata) =>{ 

        if metadata.encrypted_string.is_empty() { 
          return MetaContractResult {
            result: false,
            metadatas: Vec::new(),
            error_string: "Invalid encrypted_string cannot be empty".to_string(),
          }; 
        }

        if metadata.encrypted_symmetric_key.is_empty() { 
          return MetaContractResult {
            result: false,
            metadatas: Vec::new(),
            error_string: "encrypted_symmetric_key cannot be empty".to_string(),
          };
        }

        if metadata.auth_sig.is_any_empty_field() { 
          return MetaContractResult {
            result: false,
            metadatas: Vec::new(),
            error_string: "auth_signed fields cannot be empty".to_string(),
          };
        }
      }
      Err(_) => {
        return MetaContractResult {
          result: false,
         metadatas: Vec::new(),
           error_string: "Data does not follow the required JSON schema".to_string(),
      };
      } 
    }

    finals.push(FinalMetadata {
        public_key: transaction.public_key,
        alias: transaction.alias,
        content: transaction.data,
        version: transaction.version,
        loose: 0
    });

    MetaContractResult {
        result: true,
        metadatas: finals,
        error_string: "".to_string(),
    }
}

#[marine]
pub fn on_clone() -> bool {
    return true;
}

#[marine]
pub fn on_mint(contract: MetaContract, data_key: String, token_id: String, data: String) -> MetaContractResult {
    let mut error: Option<String> = None;
    let mut finals: Vec<FinalMetadata> = vec![];
    // extract out data

    MetaContractResult {
        result: true,
        metadatas: finals,
        error_string: "".to_string(),
    }
}

/**
 * Get data from ipfs
 */
fn get(hash: String, api_multiaddr: String, timeout_sec: u64) -> String {
  let address: String;
  let t;

  if api_multiaddr.is_empty() {
      address = DEFAULT_IPFS_MULTIADDR.to_string();
  } else {
      address = api_multiaddr;
  }

  if timeout_sec == 0 {
      t = DEFAULT_TIMEOUT_SEC;
  } else {
      t = timeout_sec;
  }

  let args = vec![String::from("dag"), String::from("get"), hash];

  let cmd = make_cmd_args(args, address, t);

  let result = ipfs(cmd);

  String::from_utf8(result.stdout).unwrap()
}

pub fn make_cmd_args(args: Vec<String>, api_multiaddr: String, timeout_sec: u64) -> Vec<String> {
  args.into_iter()
      .chain(vec![
          String::from("--timeout"),
          get_timeout_string(timeout_sec),
          String::from("--api"),
          api_multiaddr,
      ])
      .collect()
}

#[inline]
pub fn get_timeout_string(timeout: u64) -> String {
  format!("{}s", timeout)
}

// Service
// - curl

#[marine]
#[link(wasm_import_module = "host")]
extern "C" {
  pub fn ipfs(cmd: Vec<String>) -> MountedBinaryResult;
}

