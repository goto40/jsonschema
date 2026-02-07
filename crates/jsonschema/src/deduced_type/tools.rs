use crate::{
    deduced_type::{
        DeduceType, DeduceTypeError, DeduceTypeResult, DeducedType, StructAttribute, StructType,
        VariantType,
    },
    node::SchemaNode,
    properties::PropertiesValidatorsMap,
};
use std::collections::HashMap;

fn combine_structs(s1: &StructType, s2: &StructType) -> DeduceTypeResult<DeducedType> {
    let name = s1.name.clone();
    if s2.name != name {
        return Err(DeduceTypeError::unexpected(&format!(
            "combining structs with different name {:?}!={name:?}",
            s2.name
        )));
    }
    let mut attributes = s1.attributes.clone();
    for (attribute_name, attr) in &s2.attributes {
        if let Some(exisiting_attr) = attributes.get(attribute_name) {
            let new_attr = StructAttribute {
                inner_type: combine(
                    &exisiting_attr.inner_type,
                    &attr.inner_type,
                    &add_names(&name, attribute_name),
                )?,
                is_optional: exisiting_attr.is_optional && attr.is_optional,
            };
            attributes.insert(attribute_name.clone(), new_attr);
        } else {
            attributes.insert(attribute_name.clone(), attr.clone());
        }
    }
    match (&s1.additional_attributes, &s2.additional_attributes) {
        (Some(additional1), Some(additional2)) => Ok(DeducedType::Struct(Box::new(StructType {
            name: name.clone(),
            attributes,
            additional_attributes: Some(combine(
                additional1,
                additional2,
                &add_names(&name, "@additional_attribute"),
            )?),
        }))),
        (None, Some(additional)) | (Some(additional), None) => {
            Ok(DeducedType::Struct(Box::new(StructType {
                name,
                attributes,
                additional_attributes: Some(additional.clone()),
            })))
        }
        (None, None) => Ok(DeducedType::Struct(Box::new(StructType {
            name,
            attributes,
            additional_attributes: None,
        }))),
    }
}

fn add_to_variant(v: &VariantType, other: &DeducedType) -> DeducedType {
    if let DeducedType::Struct(other) = other {
        if v.possible_types.iter().any(|t| {
            if let DeducedType::Struct(s) = t {
                s.name == other.name
            } else {
                false
            }
        }) {
            return DeducedType::Variant(Box::new(v.clone()));
        }
    }
    if v.possible_types.iter().any(|t| t == other) {
        return DeducedType::Variant(Box::new(v.clone()));
    }
    let mut possible_types = [other]
        .into_iter()
        .chain(v.possible_types.iter())
        .cloned()
        .collect::<Vec<_>>();
    possible_types.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Less));
    DeducedType::Variant(Box::new(VariantType {
        name: v.name.clone(),
        possible_types,
    }))
}

/// # Errors
///
/// Will return `Err` if the type cannot be deduced.
pub fn combine(
    first: &DeducedType,
    second: &DeducedType,
    type_name: &[String],
) -> DeduceTypeResult<DeducedType> {
    match (first, second) {
        (DeducedType::Struct(s1), DeducedType::Struct(s2)) => combine_structs(s1, s2),
        (DeducedType::String, DeducedType::String) => Ok(DeducedType::String),
        (DeducedType::Boolean, DeducedType::Boolean) => Ok(DeducedType::Boolean),
        (DeducedType::Number, DeducedType::Number) => Ok(DeducedType::Number),
        (DeducedType::Integer, DeducedType::Integer) => Ok(DeducedType::Integer),
        (DeducedType::Any, other) | (other, DeducedType::Any) => Ok(other.clone()),
        (DeducedType::Variant(v), other) | (other, DeducedType::Variant(v)) => {
            Ok(add_to_variant(v, other))
        }
        (left, right) => Ok(DeducedType::Variant(Box::new(VariantType {
            name: type_name.to_vec(),
            possible_types: vec![left.clone(), right.clone()],
        }))),
    }
}

pub(crate) fn deduce_struct_type_from_properties<M: PropertiesValidatorsMap>(
    properties: &M,
    type_name: &[String],
    additional_attributes: Option<DeducedType>,
) -> DeduceTypeResult<DeducedType> {
    let attributes = properties
        .get_keys()
        .into_iter()
        .map(|name| -> DeduceTypeResult<(String, StructAttribute)> {
            let attr = StructAttribute {
                inner_type: properties
                    .get_validator(&name)
                    .ok_or(DeduceTypeError::unexpected(
                        "name not found in map (impossible)",
                    ))?
                    .deduce_type(&add_names(type_name, &name))?,
                is_optional: true,
            };
            Ok((name, attr))
        })
        .collect::<DeduceTypeResult<HashMap<_, _>>>()?;
    Ok(DeducedType::Struct(Box::new(StructType {
        name: type_name.to_owned(),
        attributes,
        additional_attributes,
    })))
}

pub(crate) fn deduce_type_from_patterns<T>(
    patterns: &[(T, SchemaNode)],
    type_name: &[String],
) -> DeduceTypeResult<DeducedType> {
    deduce_type_from_schema_nodes(patterns.iter().map(|(_, v)| v), type_name)
}

pub(crate) fn deduce_type_from_schema_nodes<'a>(
    nodes: impl Iterator<Item = &'a SchemaNode>,
    type_name: &[String],
) -> DeduceTypeResult<DeducedType> {
    nodes
        .enumerate()
        .map(|(idx, v)| {
            v.deduce_type(&add_names(
                type_name,
                &format!("@addional_attributes_{idx}"),
            ))
        })
        .reduce(|acc, next| combine(&acc?, &next?, type_name))
        .unwrap_or(Err(DeduceTypeError::unexpected(&format!(
            "empty pattern list in {type_name:?}"
        ))))
}

#[must_use]
pub fn add_names(name: &[String], added_name: &str) -> Vec<String> {
    name.iter()
        .chain([added_name.to_owned()].iter())
        .cloned()
        .collect()
}
