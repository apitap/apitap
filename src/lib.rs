//! # ApiTap
//!
//! High-performance HTTP-to-warehouse ETL engine powered by Apache DataFusion & Rust.
//!
//! ## Overview
//!
//! ApiTap enables you to:
//! - **Extract** JSON from REST APIs with smart pagination
//! - **Transform** data using SQL (Apache DataFusion)
//! - **Load** into data warehouses (PostgreSQL, with more coming)
//!
//! ## Quick Start
//!
//! ```no_run
//! use apitap::cmd::{Cli, run_pipeline};
//! use clap::Parser;
//!
//! #[tokio::main]
//! async fn main() -> apitap::Result<()> {
//!     let cli = Cli::parse();
//!     run_pipeline(&cli.modules, &cli.yaml_config).await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Features
//!
//! - **SQL Transformations**: Write transformations in SQL with Minijinja templating
//! - **Smart Pagination**: Automatic handling of limit/offset and page-based pagination (cursor coming soon)
//! - **Streaming**: Memory-efficient streaming for large datasets
//! - **Retry Logic**: Automatic retry with exponential backoff
//! - **Structured Logging**: JSON and human-readable log formats
//! - **Type Safety**: Full Rust type safety with DataFusion integration
//!
//! ## Architecture
//!
//! ```text
//! HTTP APIs → DataFusion SQL → Warehouse Writers
//! ```
//!
//! - **HTTP Layer**: Fetch data from REST APIs with pagination
//! - **Transform Layer**: Execute SQL transformations with DataFusion
//! - **Writer Layer**: Load data into target warehouses
//!
//! ## Examples
//!
//! See the `examples/` directory for:
//! - SQL transformation modules
//! - YAML configuration files
//! - Complete pipeline examples

// Public API exports
pub use errors::{ApitapError, Result};

// Public modules
pub mod cmd;
pub mod config;
pub mod errors;
pub mod http;
pub mod log;
pub mod pipeline;
pub mod utils;
pub mod writer;
