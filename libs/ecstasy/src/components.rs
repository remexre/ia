//! Some common components.

use crate::Component;
use cgmath::Point3;
use derive_more::{Display, From, Into};
use serde::{Deserialize, Serialize};

/// A dataless debug flag.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DebugFlag;

#[typetag::serde]
impl Component for DebugFlag {}

/// The name of the entity.
#[derive(
    Clone, Debug, Deserialize, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Name(pub String);

#[typetag::serde]
impl Component for Name {}

/// The position of the entity.
#[derive(Debug, Deserialize, From, Into, Serialize)]
pub struct Position(pub Point3<f32>);

impl Position {
    /// Creates a position at the given point.
    pub const fn new(x: f32, y: f32, z: f32) -> Position {
        Position(Point3::new(x, y, z))
    }
}

#[typetag::serde]
impl Component for Position {}
