//! # rs-miniredis
//!
//! A lightweight, in-memory key-value store inspired by Redis,
//! built with Rust and Tokio.
//!
//! ## As a library
//! ```rust,no_run
//! use rs_miniredis::{MiniRedisState, run_server};
//!
//! #[tokio::main]
//! async fn main() {
//!     let state = MiniRedisState::new();
//!     run_server("127.0.0.1:6379", state).await.unwrap();
//! }
//! ```

pub mod runner;
pub mod state;

pub use runner::run_server;
pub use state::State as KeyStore;
