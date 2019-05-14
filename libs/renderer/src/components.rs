//! Components used by the renderer.

use assets::{Model, Texture};
use ecstasy::Component;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// This changes the maximum draw radius from the default to the given value.
#[derive(Component, Debug, Deserialize, Serialize)]
pub struct LongDistanceDraw(pub f32);

/// Renders a mesh with an optional texture.
#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Render3D {
    /// The model whose vertices will be rendered.
    pub model: Arc<Model>,

    /// The texture that will be used to render.
    #[serde(skip)]
    pub texture: Option<Arc<Texture>>,
}
