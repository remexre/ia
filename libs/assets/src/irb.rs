//! The Ia Resource Bundle file format.

use crate::{Asset, Assets, Model};
use libremexre::{errors::Result, unwrap_arc};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::File, io::Cursor, path::Path};

/// The data in an IRB file.
///
/// This struct is bincode'd and zstd'd before being written to disk.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IRB {
    pub(crate) assets: BTreeMap<String, IRBAsset>,
}

impl IRB {
    /// Creates a new, empty IRB.
    pub fn new() -> IRB {
        IRB::default()
    }

    /// Decodes an IRB file from bytes. Prefer `load_from_file` if loading from a file.
    pub fn load_from_bytes(bs: &[u8]) -> Result<IRB> {
        bincode::deserialize(&zstd::decode_all(bs)?).map_err(From::from)
    }

    /// Decodes an IRB file from a file.
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<IRB> {
        bincode::deserialize_from(zstd::Decoder::new(File::open(path)?)?).map_err(From::from)
    }

    /// Encodes an IRB file into bytes. Prefer `save_to_file` if saving to a file.
    pub fn save_to_bytes(&self) -> Result<Vec<u8>> {
        zstd::encode_all(Cursor::new(bincode::serialize(self)?), 0).map_err(From::from)
    }

    /// Encodes an IRB file into a file.
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        bincode::serialize_into(zstd::Encoder::new(File::create(path)?, 0)?, &self)
            .map_err(From::from)
    }
}

impl From<Assets> for IRB {
    fn from(assets: Assets) -> IRB {
        let assets = assets
            .assets
            .into_iter()
            .map(|(k, v)| (k, unwrap_arc(v).into()))
            .collect();
        IRB { assets }
    }
}

/// An asset in an IRB file.
#[derive(Debug, Deserialize, Serialize)]
pub(crate) enum IRBAsset {
    FragmentShader(Vec<u8>),
    Model(Model),
    VertexShader(Vec<u8>),
}

impl From<Asset> for IRBAsset {
    fn from(asset: Asset) -> IRBAsset {
        match asset {
            Asset::FragmentShader(spirv, _) => IRBAsset::FragmentShader(spirv),
            Asset::Model(model) => IRBAsset::Model(model),
            Asset::VertexShader(spirv, _) => IRBAsset::VertexShader(spirv),
        }
    }
}
