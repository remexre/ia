//! The Ia Resource Bundle file format.

use crate::Model;
use libremexre::errors::Result;
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, collections::BTreeMap, fs::File, io::Cursor, ops::Index, path::Path};

/// The data in an IRB file.
///
/// This struct is bincode'd and zstd'd before being written to disk, but this is the data inside.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct IRB {
    assets: BTreeMap<String, ()>,
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

impl<'a, T: Ord> Index<&'a T> for IRB
where
    String: Borrow<T>,
{
    type Output = ();

    fn index(&self, key: &'a T) -> &() {
        self.assets.index(key)
    }
}

/// An asset in an IRB file.
#[derive(Debug, Default, Deserialize, Serialize)]
pub enum Asset {
    /// A model.
    Model(Model),
}
