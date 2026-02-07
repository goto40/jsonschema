use serde_json::Value;

use crate::deduced_type::{DeduceTypeError, DeduceTypeResult, DeducedType};

pub trait DeduceType {
    /// # Errors
    ///
    /// Will return `Err` if the type cannot be deduced.
    fn deduce_type(&self, type_name: &[String]) -> DeduceTypeResult<DeducedType> {
        Err(DeduceTypeError::not_implemented(&format!(
            "Trait default impl for {type_name:?}: {}",
            std::any::type_name::<Self>()
        )))
    }
}

/// # Errors
///
/// Will return `Err` if the type cannot be deduced
/// or when the schema is invalid
pub fn deduce_type(schema: &Value, type_name: &str) -> DeduceTypeResult<DeducedType> {
    crate::options()
        .should_validate_formats(true)
        .build(schema)
        .map_err(|e| DeduceTypeError::unexpected(&format!("Invalid schema: {e}")))?
        .deduce_type(&[type_name.to_owned()])
}
