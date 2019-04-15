//! A Vulkan-based renderer.
#![deny(
    bad_style,
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

mod initialize;

use std::sync::Arc;
use vulkano::{
    device::{Device, Queue},
    instance::Instance,
};

/// The main renderer value.
#[derive(Debug)]
pub struct Renderer {
    dev: Arc<Device>,
    instance: Arc<Instance>,
    queue: Arc<Queue>,
}
