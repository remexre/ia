//! A simple ECS library.
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

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod component_store;
pub mod components;
#[cfg(test)]
mod tests;

pub use crate::component_store::ComponentStore;
use std::fmt::Debug;

/// An entity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entity(usize);

/// A component. Components are data which can be attached to entities via a `ComponentStore`.
pub trait Component: 'static + Debug {}
