use std::{collections::HashMap, fmt::Display};

#[derive(Debug)]
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
    fn combine_structs(s1: &StructType, s2: &StructType) -> DeduceTypeResult<DeducedType> {
        let name = s1.name.clone();
        let mut attributes = s1.attributes.clone();
        for (name, attr) in &s2.attributes {
            if let Some(exisiting_attr) = attributes.get(name) {
                let new_attr = StructAttribute {
                    name: name.clone(),
                    inner_type: exisiting_attr.inner_type.combine(&attr.inner_type)?,
                    is_optional: exisiting_attr.is_optional && attr.is_optional,
                };
                attributes.insert(name.clone(), new_attr);
                panic!("TODO")
            } else {
                attributes.insert(name.clone(), attr.clone());
            }
        }
        Ok(DeducedType::Struct(Box::new(StructType {
            name,
            attributes,
        })))
    }

    pub fn combine(&self, other: &DeducedType) -> DeduceTypeResult<DeducedType> {
        match (self, other) {
            (DeducedType::Struct(s1), DeducedType::Struct(s2)) => Self::combine_structs(s1, s2),
            _ => Err(DeduceTypeError::not_implemented(
                "DeduceType::combine: case not implemented",
            )),
        }
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
    pub name: String,
    pub attributes: HashMap<String, StructAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructAttribute {
    pub name: String,
    pub inner_type: DeducedType,
    pub is_optional: bool,
}
