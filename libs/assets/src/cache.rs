use crate::{Model, Texture};
use std::{collections::HashMap, path::PathBuf, sync::RwLock};

lazy_static::lazy_static! {
    static ref MODELS: RwLock<HashMap<PathBuf, Model>> = RwLock::new(HashMap::new());
    static ref TEXTURES: RwLock<HashMap<PathBuf, Texture>> = RwLock::new(HashMap::new());
}
