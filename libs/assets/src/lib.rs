//! An asset manager.
//!
//! This provides components for various assets, and a central value for loading them.
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

use ecstasy::Component;
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// A model.
#[derive(Component)]
pub struct Model {}

impl Debug for Model {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Model").field("len", &()).finish()
    }
}

/// A texture.
#[derive(Component)]
pub struct Texture {}

impl Debug for Texture {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Texture").field("len", &()).finish()
    }
}
