use crate::Asset;
use ecstasy::Component;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Formatter, Result as FmtResult},
    sync::Arc,
};

/// A model.
#[derive(Component, Deserialize, Serialize)]
pub struct Model {}

impl Asset for Model {
    type Component = Arc<Model>;

    fn load_from(&self, _bs: &[u8]) -> Option<Model> {
        unimplemented!()
    }
}

impl Debug for Model {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Model").field("len", &0).finish()
    }
}
