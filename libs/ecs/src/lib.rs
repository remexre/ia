//! A simple ECS library.
//!
//! Example
//! -------
//!
//! ```rust
//! # use ecs::{components::{DebugFlag, Name}, run_system, Component, ComponentStore};
//! # use std::fmt::Write;
//! #[derive(Debug, PartialEq)]
//! struct Counter(usize);
//! impl Component for Counter {}
//!
//! let mut store = ComponentStore::new();
//!
//! let foo = store.new_entity();
//! let bar = store.new_entity();
//! let baz = store.new_entity();
//!
//! store.set_component(foo, Name("foo".to_string()));
//! store.set_component(bar, Name("bar".to_string()));
//! store.set_component(baz, Name("baz".to_string()));
//!
//! store.set_component(foo, Counter(0));
//! store.set_component(bar, Counter(0));
//!
//! store.set_component(foo, DebugFlag);
//! store.set_component(baz, DebugFlag);
//!
//! let mut n = 0;
//! while n < 25 {
//!     // For each entity with a Counter component, increment it.
//!     run_system!(store, |entity, mut ctr: Counter| {
//!         ctr.0 += 1;
//!     });
//!
//!     // Add the counter value of each entity with a Counter and a
//!     // DebugFlag to n.
//!     run_system!(store, |entity, ctr: Counter, _: DebugFlag| {
//!         n += ctr.0;
//!     });
//! }
//!
//! assert_eq!(n, 28);
//! assert_eq!(store.get_component::<Name>(foo).map(|Name(s)| s as &str), Some("foo"));
//! assert_eq!(store.get_component::<Name>(bar).map(|Name(s)| s as &str), Some("bar"));
//! assert_eq!(store.get_component::<Name>(baz).map(|Name(s)| s as &str), Some("baz"));
//! assert_eq!(store.get_component::<Counter>(foo), Some(&Counter(7)));
//! assert_eq!(store.get_component::<Counter>(bar), Some(&Counter(7)));
//! assert_eq!(store.get_component::<Counter>(baz), None);
//! assert_eq!(store.get_component::<DebugFlag>(foo), Some(&DebugFlag));
//! assert_eq!(store.get_component::<DebugFlag>(bar), None);
//! assert_eq!(store.get_component::<DebugFlag>(baz), Some(&DebugFlag));
//! ```
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
mod unsafe_option_vec;

pub use crate::component_store::ComponentStore;
pub use ecs_proc::system;
use std::fmt::Debug;

/// An entity.
///
/// This is simply an integer, wrapped up so as to preserve type safety.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entity(usize);

/// Components are data which can be attached to entities via a `ComponentStore`.
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
///
/// A second function can be provided, which takes the entity, the return value of the first
/// function and a mutable reference to the `ComponentStore`.
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
///     name.0.clone()
/// }, |entity, ret, store| {
///     let mut s = String::new();
///     for c in ret.chars().rev() {
///         s.push(c);
///     }
///     store.set_component(entity, Name(s));
/// });
///
/// assert_eq!(store.get_component::<Name>(foo).map(|Name(s)| s as &str), Some("oof"));
/// assert_eq!(store.get_component::<Name>(bar).map(|Name(s)| s as &str), Some("bar"));
/// assert_eq!(store.get_component::<Name>(baz).map(|Name(s)| s as &str), Some("zab"));
/// ```
///
/// Lastly, if a single binary function is provided, it can use `mut` in its `Component` argument
/// to take it as a mutable reference.
///
/// ```rust
/// # use ecs::{components::Name, run_system, ComponentStore};
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
/// let mut log = String::new();
/// run_system!(store, |entity, mut name: Name| {
///     let mut s = String::new();
///     for c in name.0.chars().rev() {
///         s.push(c);
///     }
///     name.0 = s;
/// });
///
/// assert_eq!(store.get_component::<Name>(foo).map(|Name(s)| s as &str), Some("oof"));
/// assert_eq!(store.get_component::<Name>(bar).map(|Name(s)| s as &str), Some("rab"));
/// assert_eq!(store.get_component::<Name>(baz).map(|Name(s)| s as &str), Some("zab"));
/// ```
#[macro_export]
macro_rules! run_system {
    ($store:expr, |$entity:ident $(, $arg:tt : $argty:ty)*| $body:block) => {{
        let store = &mut $store;
        for $entity in store.iter_entities() {
            if let ($(Some($arg),)*) = ($(store.get_component::<$argty>($entity),)*) {
                $body
            }
        }
    }};

    ($store:expr, |$entity:ident, mut $arg:tt : $argty:ty| $body:block) => {{
        let store = &mut $store;
        for $entity in store.iter_entities() {
            if let Some($arg) = store.get_mut_component::<$argty>($entity) {
                $body
            }
        }
    }};

    (
        $store:expr,
        |$entity:ident $($args:tt)*| $body:block,
        |$then_entity:ident, $val:tt, $then_store:tt| $then:block
    ) => {{
        let mut store = &mut $store;
        for entity in store.iter_entities() {
            let val = if let ($crate::run_system!(@internal @arg_pats $($rest)*)) =
                    ($crate::run_system!(@internal @arg_exprs $($rest)*)) {
                let $entity = entity;
                Some($body)
            } else {
                None
            };
            if let Some($val) = val {
                let $then_entity = entity;
                let $then_store = &mut store;
                $then
            }
        }
    }};

    (@internal @arg_pats , $name:ident : $ty:ty) => {
        Some($name)
    };
    (@internal @arg_pats , $name:ident : $ty:ty , $($rest:tt)*) => {
        Some($name), run_system!(@internal @arg_pats , $($rest)*)
    };

    (@internal @arg_exprs , $name:ident : $ty:ty) => {
        store.get_component::<$ty>(entity)
    };
    (@internal @arg_exprs , $name:ident : $ty:ty , $($rest:tt)*) => {
        store.get_component::<$ty>(entity), run_system!(@internal @arg_exprs , $($rest)*)
    };
}

#[cfg(test)]
mod tests;
