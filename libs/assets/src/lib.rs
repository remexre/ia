//! An asset manager.
//!
//! This provides a store of various assets.
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
mod model;

use crate::irb::{IRBAsset, IRB};
pub use crate::model::Model;
use derive_more::Index;
use libremexre::errors::Result;
use std::{collections::BTreeMap, error::Error, sync::Arc};
use vulkano::{device::Device, pipeline::shader::ShaderModule};

/// The assets loaded from an `IRB`.
#[derive(Debug, Index)]
pub struct Assets {
    assets: BTreeMap<String, Arc<Asset>>,
}

impl Assets {
    /// Loads assets from an `IRB`, instantiating them on the given Vulkan device. Assets that fail
    /// to load will not be present in the `Assets` object, and will instead produce errors.
    pub fn from_irb(
        irb: IRB,
        device: Arc<Device>,
    ) -> (Assets, Vec<Box<dyn Error + Send + Sync + 'static>>) {
        let mut assets = BTreeMap::new();
        let mut errs = Vec::new();
        for (name, asset) in irb.assets {
            match Asset::from_irb(asset, &device) {
                Ok(asset) => {
                    let _ = assets.insert(name, Arc::new(asset));
                }
                Err(err) => errs.push(err),
            }
        }
        (Assets { assets }, errs)
    }
}

/// A single asset.
#[derive(Clone, Debug)]
pub enum Asset {
    /// A fragment shader.
    FragmentShader(Vec<u8>, Arc<ShaderModule>),

    /// A model.
    Model(Model),

    /// A vertex shader.
    VertexShader(Vec<u8>, Arc<ShaderModule>),
}

impl Asset {
    fn from_irb(asset: IRBAsset, device: &Arc<Device>) -> Result<Asset> {
        Ok(match asset {
            IRBAsset::FragmentShader(spirv) => {
                let sm = unsafe { ShaderModule::new(device.clone(), &spirv)? };
                Asset::FragmentShader(spirv, sm)
            }
            IRBAsset::Model(model) => Asset::Model(model),
            IRBAsset::VertexShader(spirv) => {
                let sm = unsafe { ShaderModule::new(device.clone(), &spirv)? };
                Asset::VertexShader(spirv, sm)
            }
        })
    }
}
