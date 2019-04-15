//! Some common components.

use crate::Component;
use cgmath::Point3;
use derive_more::{Display, From, Into};

/// The name of the entity.
#[derive(Debug, Display, From, Into)]
pub struct Name(pub String);

impl Component for Name {}

/// The position of the entity.
#[derive(Debug, From, Into)]
pub struct Position(pub Point3<f32>);

impl Position {
    /// Creates a position at the given point.
    pub const fn new(x: f32, y: f32, z: f32) -> Position {
        Position(Point3::new(x, y, z))
    }
}

impl Component for Position {}
