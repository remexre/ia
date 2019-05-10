use crate::{asset_sealed::AssetSealed, loader::AssetKind};
use derive_more::From;
use ecstasy::Component;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

/// A model.
#[derive(Component, Debug, Deserialize, From, Serialize)]
pub struct Model(Arc<ModelInner>);

impl AssetSealed for Model {
    type Component = Model;
    type Inner = ModelInner;
    type LoadFromError = Box<dyn Error>;
    const KIND: AssetKind = AssetKind::Model;

    fn load_from(_bs: &[u8]) -> Result<ModelInner, Box<dyn Error>> {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModelInner {}
