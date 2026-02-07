use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum DeduceTypeError {
    NotImplemented(String),
    Unexpected(String),
}
impl DeduceTypeError {
    #[must_use]
    pub fn not_implemented(what: &str) -> Self {
        DeduceTypeError::NotImplemented(what.into())
    }
    #[must_use]
    pub fn unexpected(what: &str) -> Self {
        DeduceTypeError::Unexpected(what.into())
    }
}
impl Display for DeduceTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for DeduceTypeError {}
pub type DeduceTypeResult<T> = std::result::Result<T, DeduceTypeError>;
