//! An asset manager.
//!
//! This provides components for various assets, and a central system for loading them.
#![deny(
    bad_style,
    bare_trait_objects,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unions_with_drop_fields,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    while_true
)]

mod loader;
mod model;
mod texture;

pub use crate::{loader::Loader, model::Model, texture::Texture};
use std::sync::Arc;

/// A common trait for loadable assets.
trait Asset: 'static + Sized {
    type Component: From<Arc<Self>>;

    fn load_from(&self, bs: &[u8]) -> Option<Self>;
}
