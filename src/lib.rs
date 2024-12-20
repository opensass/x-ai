#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod api_key;
pub mod chat_compl;
pub mod client;
pub mod completions;
pub mod embedding;
pub mod embedding_get;
pub mod embedding_mod;
pub mod error;
pub mod get_mod;
pub mod lang_mod;
pub mod list_lang_mod;
pub mod list_mod;
pub mod traits;

pub const XAI_V1_URL: &str = "https://api.x.ai/v1";
