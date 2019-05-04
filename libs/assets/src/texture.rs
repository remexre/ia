use crate::Asset;
use ecstasy::Component;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// A texture.
#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Texture {}

impl Asset for Texture {
    type Component = Arc<Texture>;

    fn load_from(&self, _bs: &[u8]) -> Option<Texture> {
        unimplemented!()
    }
}
