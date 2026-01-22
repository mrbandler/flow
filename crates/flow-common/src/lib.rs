//! Common utilities shared across Flow crates.
//!
//! This crate provides shared functionality used by multiple Flow crates,
//! including path manipulation utilities and other cross-cutting concerns.
//!
//! # Re-exports
//!
//! - [`PathExt`] - Extension trait for [`std::path::Path`] with normalization support.

mod path;

pub use path::PathExt;
