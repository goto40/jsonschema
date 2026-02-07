use crate::deduced_type::{DeduceTypeError, DeduceTypeResult, DeducedType};

pub trait DeduceType {
    /// # Errors
    ///
    /// Will return `Err` if the type cannot be deduced.
    fn deduce_type(&self, type_name: &str) -> DeduceTypeResult<DeducedType> {
        Err(DeduceTypeError::not_implemented(&format!(
            "Trait default impl for {type_name}: {}",
            std::any::type_name::<Self>()
        )))
    }
}
