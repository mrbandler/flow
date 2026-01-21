//! Space metadata types.
//!
//! This module provides [`Metadata`], which contains information about
//! a space such as its name and the Flow version that created it.
//!
//! # Overview
//!
//! Every Flow space has an associated metadata file (`.flow/space.json`)
//! that stores essential information about the space. This metadata is
//! read when loading a space and written when initializing a new one.
//!
//! # Serialization
//!
//! The [`Metadata`] struct is serialized to JSON using [`serde`]. The
//! format is designed to be human-readable and forward-compatible,
//! allowing future versions of Flow to add new fields without breaking
//! existing spaces.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// Metadata about a Flow space.
///
/// This struct is serialized to JSON and stored in the `.flow/space.json`
/// file within each space. It contains essential information needed to
/// identify and manage the space.
///
/// # Fields
///
/// * `name` - The human-readable name of the space, used for identification.
/// * `version` - The Flow version that created or last migrated this space.
///
/// # Serialization Format
///
/// The metadata is stored as JSON:
///
/// ```json
/// {
///     "name": "personal",
///     "version": "0.1.0"
/// }
/// ```
///
/// # Examples
///
/// Creating metadata for a new space:
///
/// ```ignore
/// use flow_core::space::Metadata;
///
/// let metadata = Metadata {
///     name: "personal".to_string(),
///     version: "0.1.0".to_string(),
/// };
/// ```
///
/// Serializing metadata to JSON:
///
/// ```ignore
/// use flow_core::space::Metadata;
///
/// let metadata = Metadata {
///     name: "work".to_string(),
///     version: "0.1.0".to_string(),
/// };
///
/// let json = serde_json::to_string_pretty(&metadata).unwrap();
/// assert!(json.contains("\"name\": \"work\""));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    /// The human-readable name of the space.
    ///
    /// This name is used to identify the space in commands like `flow open <name>`.
    pub name: String,

    /// The version of Flow that created this space.
    ///
    /// This can be used for future migrations if the space format changes.
    pub version: Cow<'static, str>,
}
