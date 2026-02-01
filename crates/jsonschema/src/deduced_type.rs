use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
pub enum DeduceTypeError {
    NotImplemented(String),
    Unexpected(String),
}
impl DeduceTypeError {
    pub fn not_implemented(what: &str) -> Self {
        DeduceTypeError::NotImplemented(what.into())
    }
    pub fn unexpected(what: &str) -> Self {
        DeduceTypeError::Unexpected(what.into())
    }
}
impl Display for DeduceTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for DeduceTypeError {}
pub type DeduceTypeResult<T> = std::result::Result<T, DeduceTypeError>;

#[derive(Debug, Clone, PartialEq)]
pub enum DeducedType {
    String,
    Number,
    Boolean,
    Array(Box<ArrayType>),
    Struct(Box<StructType>),
    Variant(Box<VariantType>),
    Any,
}

impl DeducedType {
    pub fn combine(&self, _other: DeducedType) -> DeduceTypeResult<DeducedType> {
        Err(DeduceTypeError::not_implemented("DeduceType::combine")) // TODO
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    inner_type: DeducedType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariantType {
    attributes: Vec<DeducedType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    name: String,
    attributes: HashMap<String, StructAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructAttribute {
    name: String,
    inner_type: DeducedType,
    is_optional: bool,
}
