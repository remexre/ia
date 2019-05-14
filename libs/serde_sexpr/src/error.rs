/// An error while deserializing.
#[derive(Debug)]
pub struct Error;

/// A convenient alias for `Result`.
pub type Result<T> = std::result::Result<T, Error>;
