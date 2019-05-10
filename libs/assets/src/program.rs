use crate::Asset;
use derive_more::From;
use ecstasy::Component;
use serde::{Deserialize, Serialize};
use std::{error::Error, sync::Arc};

/// A program.
#[derive(Component, Debug, Deserialize, From, Serialize)]
pub struct Program(Arc<ProgramInner>);

impl Asset for Program {
    type Component = Program;
    type Inner = ProgramInner;
    type LoadFromError = Box<dyn Error>;

    fn load_from(_bs: &[u8]) -> Result<ProgramInner, Box<dyn Error>> {
        unimplemented!()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgramInner {
    // TODO
    /// A promise that the program is safe and correct.
    pub promise: ProgramSafetyPromise,
}

/// A promise that a SPIR-V program is safe and correct and all that.
#[derive(Debug)]
pub struct ProgramSafetyPromise(());

impl ProgramSafetyPromise {
    pub unsafe fn i_promise() -> ProgramSafetyPromise {
        ProgramSafetyPromise(())
    }
}
