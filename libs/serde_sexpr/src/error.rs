/// An error while deserializing.
#[derive(Debug)]
pub enum Error {
    /// A string failed to parse as an s-expression.
    ParseFailed,

    /// An s-expression was successfully parsed, but there was trailing input.
    ParseTrailing,
}

/// A convenient alias for `Result`.
pub type Result<T> = std::result::Result<T, Error>;
