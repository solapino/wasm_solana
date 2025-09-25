#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/readme.md"))]

pub use solana_account_decoder_client_types_wasm as solana_account_decoder_client_types;
pub use solana_account_decoder_wasm as solana_account_decoder;
pub use solana_transaction_status_client_types_wasm as solana_transaction_status_client_types;
pub use solana_transaction_status_wasm as solana_transaction_status;

pub use crate::client::*;
pub use crate::constants::*;
pub use crate::errors::*;
pub use crate::extensions::*;
pub use crate::methods::*;
pub use crate::providers::*;
pub use crate::rpc_config::*;
pub use crate::solana_client::*;
pub use crate::utils::spawn_local;

mod client;
mod constants;
mod errors;
mod extensions;
mod methods;
pub mod nonce_utils;
mod providers;
pub mod rpc_config;
pub mod rpc_filter;
pub mod rpc_response;
pub mod runtime;
mod solana_client;
pub mod utils;

pub mod prelude {
	pub use futures::FutureExt;
	pub use futures::SinkExt;
	pub use futures::StreamExt;
	pub use futures::TryFutureExt;
	pub use futures::TryStreamExt;
	pub use wallet_standard::prelude::*;

	pub use crate::RpcProvider;
	pub use crate::extensions::VersionedMessageExtension;
	pub use crate::extensions::VersionedTransactionExtension;
}
