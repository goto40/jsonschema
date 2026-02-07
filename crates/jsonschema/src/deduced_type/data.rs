use std::collections::HashMap;
#[derive(Debug, Clone, PartialEq)]
pub enum DeducedType {
    String,
    Number,
    Boolean,
    Enum(Box<EnumType>),
    Array(Box<ArrayType>),
    Struct(Box<StructType>),
    Variant(Box<VariantType>),
    Any,
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
pub struct EnumType {
    pub name: String,
    pub enum_entries: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructAttribute {
    pub inner_type: DeducedType,
    pub is_optional: bool,
}
