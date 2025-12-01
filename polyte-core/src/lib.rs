//! # polyte-core
//!
//! Core utilities and shared types for Polyte Polymarket API clients.
//!
//! This crate provides common functionality used across `polyte-clob` and `polyte-gamma`:
//! - Shared error types and error handling
//! - HTTP client configuration
//! - Request builder utilities

pub mod client;
pub mod error;
pub mod request;

pub use client::{ClientBuilder, ClientConfig};
pub use error::ApiError;
pub use request::{QueryBuilder, Request, RequestError};
