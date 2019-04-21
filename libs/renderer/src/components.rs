//! Components used by the renderer.

use ecs::Component;

/// This changes the maximum draw radius from the default to the given value.
#[derive(Component, Debug)]
pub struct LongDistanceDraw(pub f32);
