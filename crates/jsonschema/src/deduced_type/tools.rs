use crate::deduced_type::{
    DeduceTypeError, DeduceTypeResult, DeducedType, StructAttribute, StructType,
};

fn combine_structs(s1: &StructType, s2: &StructType) -> DeduceTypeResult<DeducedType> {
    let name = s1.name.clone();
    if s2.name != name {
        return Err(DeduceTypeError::unexpected(&format!(
            "combining structs with different name {}!={name}",
            s2.name
        )));
    }
    let mut attributes = s1.attributes.clone();
    for (name, attr) in &s2.attributes {
        if let Some(exisiting_attr) = attributes.get(name) {
            let new_attr = StructAttribute {
                inner_type: combine(&exisiting_attr.inner_type, &attr.inner_type)?,
                is_optional: exisiting_attr.is_optional && attr.is_optional,
            };
            attributes.insert(name.clone(), new_attr);
        } else {
            attributes.insert(name.clone(), attr.clone());
        }
    }
    Ok(DeducedType::Struct(Box::new(StructType {
        name,
        attributes,
    })))
}

/// # Errors
///
/// Will return `Err` if the type cannot be deduced.
pub fn combine(me: &DeducedType, other: &DeducedType) -> DeduceTypeResult<DeducedType> {
    match (me, other) {
        (DeducedType::Struct(s1), DeducedType::Struct(s2)) => combine_structs(s1, s2),
        (DeducedType::Any, other) => Ok(other.clone()),
        (me, DeducedType::Any) => Ok(me.clone()),
        _ => Err(DeduceTypeError::not_implemented(&format!(
            "DeduceType::combine: case not implemented for ({me:?},{other:?})"
        ))),
    }
}
