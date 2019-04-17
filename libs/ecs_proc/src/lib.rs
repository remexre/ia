//! Proc macros for the `ecs` crate.
#![deny(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    missing_debug_implementations,
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

// Rust bug #42008
// #![deny(missing_docs)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;

/// Creates an implementation of the `System` trait from a closure expression.
///
/// ```
/// # use ecs::Component;
/// # use ecs_proc::system;
/// # #[derive(Debug)]
/// # struct FooComponent(u32);
/// # impl Component for FooComponent {}
/// # #[derive(Debug)]
/// # struct BarComponent;
/// # impl Component for BarComponent {}
/// system!(|entity, foo: &FooComponent, bar: &BarComponent| {
///     println!("{:?} has foo={:?}, bar={:?}", entity, foo, bar);
/// });
///
/// system!(|entity, foo: &mut FooComponent| {
///     foo.0 *= 2;
/// });
/// ```
#[proc_macro]
pub fn system(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::Expr);
    TokenStream::from(quote! {
        #input
    })
}
