use std::{cmp::Ordering, collections::HashMap};
#[derive(Debug, Clone, PartialEq)]
pub enum DeducedType {
    String,
    Number,
    Integer,
    Boolean,
    Enum(Box<EnumType>),
    Array(Box<ArrayType>),
    Struct(Box<StructType>),
    Variant(Box<VariantType>),
    Any,
    Null,
}

impl DeducedType {
    pub fn index(&self) -> usize {
        match self {
            DeducedType::String => 0,
            DeducedType::Number => 1,
            DeducedType::Integer => 2,
            DeducedType::Boolean => 3,
            DeducedType::Enum(_) => 4,
            DeducedType::Array(_) => 5,
            DeducedType::Struct(_) => 6,
            DeducedType::Variant(_) => 7,
            DeducedType::Any => 8,
            DeducedType::Null => 9,
        }
    }
}

impl PartialOrd for DeducedType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.index().cmp(&other.index()) {
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Equal => match (self, other) {
                (DeducedType::Enum(e1), DeducedType::Enum(e2)) => e1.name.partial_cmp(&e2.name),
                (DeducedType::Array(a1), DeducedType::Array(a2)) => {
                    a1.inner_type.partial_cmp(&a2.inner_type)
                }
                (DeducedType::Struct(s1), DeducedType::Struct(s2)) => s1.name.partial_cmp(&s2.name),
                (DeducedType::Variant(v1), DeducedType::Variant(v2)) => {
                    v1.name.partial_cmp(&v2.name)
                }
                _ => Some(Ordering::Equal),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    pub inner_type: DeducedType,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariantType {
    pub name: Vec<String>,
    pub possible_types: Vec<DeducedType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructType {
    pub name: Vec<String>,
    pub attributes: HashMap<String, StructAttribute>,
    pub additional_attributes: Option<DeducedType>, // None = no additional fields allowed (can be overridden by non-None specifications)
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumType {
    pub name: Vec<String>,
    pub enum_entries: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructAttribute {
    pub inner_type: DeducedType,
    pub is_optional: bool,
}
