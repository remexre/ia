//! Some common components.

use crate::Component;
use cgmath::Point3;
use derive_more::{Display, From, Into};

/// A dataless debug flag.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DebugFlag;

impl Component for DebugFlag {}

/// The name of the entity.
#[derive(Clone, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd)]
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
