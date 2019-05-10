use crate::{asset_sealed::AssetSealed, loader::AssetKind};
use derive_more::From;
use ecstasy::Component;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

/// A texture.
#[derive(Component, Debug, Deserialize, From, Serialize)]
pub struct Texture(Arc<TextureInner>);

impl AssetSealed for Texture {
    type Component = Texture;
    type Inner = TextureInner;
    type LoadFromError = Box<dyn Error>;
    const KIND: AssetKind = AssetKind::Texture;

    fn load_from(_bs: &[u8]) -> Result<TextureInner, Box<dyn Error>> {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TextureInner {}
