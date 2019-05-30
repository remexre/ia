//! A simple ECS library.
//!
//! Example
//! -------
//!
//! ```
//! # use ecstasy::{
//! #     components::{DebugFlag, Name},
//! #     system, system_mut, Component, ComponentStore, Engine, Entity, System,
//! # };
//! # use serde::{Deserialize, Serialize};
//! # use std::sync::atomic::{AtomicUsize, Ordering};
//! #[derive(Component, Debug, Deserialize, PartialEq, Serialize)]
//! struct Counter(usize);
//!
//! #[system(simple)]
//! fn AssertNameHas3Bytes(entity: Entity, _dt: f32, name: &Name) {
//!     assert_eq!(name.0.len(), 3, "{:?}'s name ({}) should have been 3 bytes", entity, name);
//! }
//!
//! #[system_mut(simple)]
//! fn IncrCounter(entity: Entity, _dt: f32, counter: &mut Counter) {
//!     counter.0 += 1;
//! }
//!
//! struct SumDebugCounters<'a>(&'a AtomicUsize);
//! impl<'a> System for SumDebugCounters<'a> {
//!     fn run(&mut self, cs: &ComponentStore, _dt: f32) {
//!         cs.iter_entities().for_each(|entity| {
//!             if let (Some(counter), Some(_)) =
//!                     (cs.get_component::<Counter>(entity), cs.get_component::<DebugFlag>(entity)) {
//!                 self.0.fetch_add(counter.0, Ordering::SeqCst);
//!             }
//!         })
//!     }
//! }
//!
//! let mut n = AtomicUsize::new(0);
//! let mut engine = Engine::new()
//!     .add_mut_pass(IncrCounter)
//!     .build_par_pass()
//!         .add(AssertNameHas3Bytes)
//!         .add(SumDebugCounters(&n))
//!     .finish();
//!
//! let foo = engine.store.new_entity();
//! let bar = engine.store.new_entity();
//! let baz = engine.store.new_entity();
//!
//! engine.store.set_component(foo, Name("foo".to_string()));
//! engine.store.set_component(bar, Name("bar".to_string()));
//! engine.store.set_component(baz, Name("baz".to_string()));
//!
//! engine.store.set_component(foo, Counter(0));
//! engine.store.set_component(bar, Counter(0));
//!
//! engine.store.set_component(foo, DebugFlag);
//! engine.store.set_component(baz, DebugFlag);
//!
//! while n.load(Ordering::SeqCst) < 25 {
//!     engine.run_once();
//! }
//!
//! assert_eq!(engine.store.get_component::<Name>(foo).map(|Name(s)| &s as &str), Some("foo"));
//! assert_eq!(engine.store.get_component::<Name>(bar).map(|Name(s)| &s as &str), Some("bar"));
//! assert_eq!(engine.store.get_component::<Name>(baz).map(|Name(s)| &s as &str), Some("baz"));
//! assert_eq!(engine.store.get_component::<Counter>(foo), Some(&Counter(7)));
//! assert_eq!(engine.store.get_component::<Counter>(bar), Some(&Counter(7)));
//! assert_eq!(engine.store.get_component::<Counter>(baz), None);
//! assert_eq!(engine.store.get_component::<DebugFlag>(foo), Some(&DebugFlag));
//! assert_eq!(engine.store.get_component::<DebugFlag>(bar), None);
//! assert_eq!(engine.store.get_component::<DebugFlag>(baz), Some(&DebugFlag));
//! assert_eq!(n.load(Ordering::SeqCst), 28);
//! ```
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

#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

mod component_store;
pub mod components;
mod engine;
mod unsafe_option_vec;

pub use crate::{
    component_store::ComponentStore,
    engine::{Engine, EnginePassBuilder},
};
pub use ecstasy_proc_macros::{system, system_mut, Component};
use std::{fmt::Debug, num::NonZeroUsize};

/// An entity.
///
/// This is an integer, wrapped up so as to preserve type safety.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Entity(NonZeroUsize);

/// Components are data which can be attached to entities via a `ComponentStore`.
///
/// This trait can be derived:
///
/// ```
/// # use serde::{Deserialize, Serialize};
/// use ecstasy::Component;
///
/// #[derive(Component, Debug, Deserialize, Serialize)]
/// struct Foo(u32, isize);
/// ```
#[typetag::serde(tag = "t")]
pub trait Component: 'static + Debug + Send + Sync {}

/// A system that does not modify the `ComponentStore`. These systems can be run in parallel with
/// *each other*, but should generally not use parallelism internally.
pub trait System: Send {
    /// Runs the system.
    ///
    /// `dt` is in seconds.
    fn run(&mut self, cs: &ComponentStore, dt: f32);
}

impl<T: ?Sized + System> System for Box<T> {
    fn run(&mut self, cs: &ComponentStore, dt: f32) {
        (**self).run(cs, dt)
    }
}

/// A system that modifies the `ComponentStore`.
pub trait SystemMut {
    /// Runs the system.
    ///
    /// `dt` is in seconds.
    fn run(&mut self, cs: &mut ComponentStore, dt: f32);
}

impl<T: ?Sized + SystemMut> SystemMut for Box<T> {
    fn run(&mut self, cs: &mut ComponentStore, dt: f32) {
        (**self).run(cs, dt)
    }
}

#[cfg(test)]
mod tests;
