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

#[cfg(test)]
mod tests;

mod component_store;
pub mod components;
mod unsafe_option_vec;

pub use crate::component_store::ComponentStore;
use std::fmt::Debug;

/// An entity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entity(usize);

/// A component. Components are data which can be attached to entities via a `ComponentStore`.
pub trait Component: 'static + Debug + Send + Sync {}

/// Runs the given system on all entities that have all the relevant components.
///
/// An unary function will be called for every entity:
///
/// ```rust
/// # use ecs::{components::DebugFlag, run_system, ComponentStore};
/// let mut store = ComponentStore::new();
/// store.new_entity();
/// store.new_entity();
/// store.new_entity();
///
/// let mut n = 0;
/// run_system!(store, |_entity| {
///     n += 1;
/// });
/// assert_eq!(n, 3);
/// ```
///
/// An `n`-ary function (where `n` > 2) will be called for every entity that has all of the
/// specified components.
///
/// ```rust
/// # use ecs::{components::{DebugFlag, Name}, run_system, ComponentStore};
/// # use std::fmt::Write;
/// let mut store = ComponentStore::new();
///
/// let foo = store.new_entity();
/// let bar = store.new_entity();
/// let baz = store.new_entity();
///
/// store.set_component(foo, Name("foo".to_string()));
/// store.set_component(bar, Name("bar".to_string()));
/// store.set_component(baz, Name("baz".to_string()));
///
/// store.set_component(foo, DebugFlag);
/// store.set_component(baz, DebugFlag);
///
/// let mut log = String::new();
/// run_system!(store, |entity, _: DebugFlag, name: Name| {
///     writeln!(log, "{:?} {:?}", entity, name);
/// });
/// assert_eq!(
///     log,
///     concat![
///         "Entity(0) Name(\"foo\")\n",
///         "Entity(2) Name(\"baz\")\n",
///     ],
/// );
/// ```
#[macro_export]
macro_rules! run_system {
    ($store:expr, |$entity:ident $(, $arg:tt : $argty:ty)*| $body:block) => {{
        let store = $store;
        for $entity in store.iter_entities() {
            if let ($(Some($arg),)*) = ($(store.get_component::<$argty>($entity),)*) {
                $body
            }
        }
    }};
}
