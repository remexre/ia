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

pub mod irb;
mod loader;
mod model;
mod program;
mod texture;

use crate::{
    asset_sealed::AssetSealed,
    loader::{AssetRequest, AssetRequests},
};
pub use crate::{
    loader::Loader,
    model::Model,
    program::{Program, ProgramInner, ProgramSafetyPromise},
    texture::Texture,
};
use ecstasy::{ComponentStore, Entity};
use std::path::PathBuf;

mod asset_sealed {
    use crate::loader::AssetKind;
    use std::{fmt::Display, sync::Arc};

    pub trait AssetSealed: 'static + Sized {
        type Component: From<Arc<Self::Inner>>;
        type Inner;
        type LoadFromError: Display;

        const KIND: AssetKind;

        fn load_from(bs: &[u8]) -> Result<Self::Inner, Self::LoadFromError>;
    }
}

/// A common trait for loadable assets.
pub trait Asset: AssetSealed {}

impl<T: AssetSealed> Asset for T {}

/// An extension trait for requesting an asset.
pub trait AssetRequestExt {
    /// Inserts a request for an asset by path.
    fn request_asset<T: Asset>(&mut self, entity: Entity, path: PathBuf);
}

impl AssetRequestExt for ComponentStore {
    fn request_asset<T: Asset>(&mut self, entity: Entity, path: PathBuf) {
        self.get_mut_component::<AssetRequests>(entity)
            .get_or_insert_with(AssetRequests::default)
            .0
            .push(AssetRequest {
                kind: T::KIND,
                path,
            })
    }
}
