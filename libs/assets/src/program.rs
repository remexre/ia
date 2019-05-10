use crate::Asset;
use derive_more::From;
use ecstasy::Component;
use serde::{
    de::{Error as DeError, Unexpected},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    error::Error,
    fmt::{Formatter, Result as FmtResult},
    sync::Arc,
};

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
    const STR: &'static str = "I promise this is safe!";

    pub unsafe fn i_promise() -> ProgramSafetyPromise {
        ProgramSafetyPromise(())
    }
}

impl<'de> Deserialize<'de> for ProgramSafetyPromise {
    fn deserialize<D>(deserializer: D) -> Result<ProgramSafetyPromise, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = ProgramSafetyPromise;

            fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
                formatter.write_str("the string \"I promise this is safe!\"")
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<ProgramSafetyPromise, E> {
                if value == ProgramSafetyPromise::STR {
                    Ok(ProgramSafetyPromise(()))
                } else {
                    Err(E::invalid_value(
                        Unexpected::Str(value),
                        &ProgramSafetyPromise::STR,
                    ))
                }
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl Serialize for ProgramSafetyPromise {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(ProgramSafetyPromise::STR)
    }
}
