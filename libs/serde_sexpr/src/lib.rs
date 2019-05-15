//! Serde support for S-Expressions.
#![deny(
    bad_style,
    bare_trait_objects,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    missing_debug_implementations,
    missing_docs,
    no_mangle_generic_items,
    non_shorthand_field_patterns,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    trivial_casts,
    trivial_numeric_casts,
    unconditional_recursion,
    unions_with_drop_fields,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_extern_crates,
    unused_import_braces,
    unused_parens,
    unused_qualifications,
    unused_results,
    while_true
)]

mod de;
mod error;
mod parser;
mod ser;
#[cfg(test)]
mod tests;

pub use crate::{
    de::Deserializer,
    error::{Error, Result},
    ser::Serializer,
};
use std::fmt::{Display, Formatter, Result as FmtResult};

/// An s-expression.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Value {
    /// A list.
    List(Vec<Value>),

    /// A symbol.
    Sym(String),
}

impl Display for Value {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            Value::List(l) => {
                write!(fmt, "(")?;
                let mut first = true;
                for x in l {
                    let sep = if first {
                        first = false;
                        ""
                    } else {
                        " "
                    };
                    write!(fmt, "{}{}", sep, x)?;
                }
                write!(fmt, ")")
            }
            Value::Sym(s) => {
                if s.chars().any(needs_quoting) || s.is_empty() {
                    write!(fmt, "|")?;
                    for ch in s.chars() {
                        let prefix = if needs_quoting(ch) { "\\" } else { "" };
                        write!(fmt, "{}{}", prefix, ch)?;
                    }
                    write!(fmt, "|")
                } else {
                    write!(fmt, "{}", s)
                }
            }
        }
    }
}

impl From<String> for Value {
    fn from(s: String) -> Value {
        Value::Sym(s)
    }
}

impl From<Vec<Value>> for Value {
    fn from(l: Vec<Value>) -> Value {
        Value::List(l)
    }
}

/// Returns whether the given character needs quoting.
fn needs_quoting(ch: char) -> bool {
    ch.is_whitespace() || "()|\\".contains(ch)
}
