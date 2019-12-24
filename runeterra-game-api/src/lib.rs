//! Proxy API for the Legends of Runeterra [`Game Client API`]
//!
//! [`Game Client API`]: https://developer.riotgames.com/docs/lor#game-client-api
//!
//! # Examples
//! ```
//! use runeterra_game_api::{Client, DEFAULT_PORT};
//! #[tokio::main]
//! async fn main() {
//!    let client = Client::new(DEFAULT_PORT);
//!    let result = client.get_static_decklist().await;
//!    dbg!(&result);
//! }
//!```
//!
mod client;
mod error;

pub use self::client::*;
pub use self::error::*;
