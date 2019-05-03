use crate::{Model, Texture};
use antidote::RwLock;
use libremexre::errors::Result;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

macro_rules! cache {
    ($ty:ident, $name:ident, $name_str:literal) => {
        mod $name {
            use super::*;

            lazy_static::lazy_static! {
                static ref CACHE: RwLock<HashMap<PathBuf, Arc<$ty>>> = RwLock::new(HashMap::new());
            }

            impl $ty {
                #[doc = "Clears the "]
                #[doc = $name_str]
                #[doc = " cache."]
                pub fn clear_cache() {
                    CACHE.write().clear()
                }

                #[doc = "Loads a "]
                #[doc = $name_str]
                #[doc = " by path. The "]
                #[doc = $name_str]
                #[doc = " is cached; use `clear_cache()` to clear it."]
                pub fn load_from(path: &Path) -> Result<Arc<$ty>> {
                    if let Some(val) = CACHE.read().get(path) {
                        Ok(val.clone())
                    } else {
                        let cache = CACHE.write();
                        unimplemented!("{:?}", *cache)
                    }
                }
            }
        }
    };
}

cache!(Model, model, "model");
cache!(Texture, texture, "texture");
