use crate::{asset_sealed::AssetSealed, loader::AssetKind};
use bincode::deserialize;
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

impl AssetSealed for Program {
    type Component = Program;
    type Inner = ProgramInner;
    type LoadFromError = Box<dyn Error>;
    const KIND: AssetKind = AssetKind::Program;

    fn load_from(bs: &[u8]) -> Result<ProgramInner, Box<dyn Error>> {
        deserialize(bs).map_err(|err| -> Box<dyn Error> { Box::new(err) })
    }
}

/// The actual data of a `Program`.
#[derive(Debug, Deserialize, Serialize)]
pub struct ProgramInner {
    /// The bytes of the compiled vertex shader.
    pub vert_bytes: Vec<u8>,

    /// The bytes of the compiled fragment shader.
    pub frag_bytes: Vec<u8>,

    /// A promise that the program is safe and correct.
    pub promise: ProgramSafetyPromise,
}

/// A promise that a SPIR-V program is safe and correct and all that.
///
/// ```
/// # use assets::ProgramSafetyPromise;
/// # use bincode::{deserialize, serialize};
/// let promise = unsafe { ProgramSafetyPromise::i_promise() };
/// let bs = serialize(&promise).unwrap();
/// assert_eq!(bs, b"\x17\x00\x00\x00\x00\x00\x00\x00I promise this is safe!");
/// let promise2 = deserialize(&bs).unwrap();
/// assert_eq!(promise, promise2);
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct ProgramSafetyPromise(());

impl ProgramSafetyPromise {
    const STR: &'static str = "I promise this is safe!";

    /// Creates a `ProgramSafetyPromise`. You promise.
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

            fn expecting(&self, fmt: &mut Formatter) -> FmtResult {
                write!(fmt, "the string \"{}\"", ProgramSafetyPromise::STR)
            }

            fn visit_borrowed_str<E: DeError>(
                self,
                value: &'de str,
            ) -> Result<ProgramSafetyPromise, E> {
                self.visit_str(value)
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
