use crate::Renderer;
use std::error::Error;
use vulkano::instance::{Instance, InstanceExtensions};

impl Renderer {
    /// Creates a new `Renderer`.
    pub fn new() -> Result<Renderer, Box<dyn Error>> {
        let instance = Instance::new(None, &InstanceExtensions::none(), None)?;
        unimplemented!("{:?}", instance)
    }
}
