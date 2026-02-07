use std::collections::HashMap;

use crate::{
    compiler,
    deduced_type::{self, DeduceType, DeduceTypeResult, DeducedType, StructType},
    error::{no_error, ErrorIterator, ValidationError},
    keywords::CompilationResult,
    paths::{LazyLocation, Location, RefTracker},
    types::JsonType,
    validator::{Validate, ValidationContext},
};
use serde_json::{Map, Value};

pub(crate) struct RequiredValidator {
    required: Vec<String>,
    location: Location,
}

impl RequiredValidator {
    #[inline]
    pub(crate) fn compile(items: &[Value], location: Location) -> CompilationResult<'_> {
        let mut required = Vec::with_capacity(items.len());
        for item in items {
            match item {
                Value::String(string) => required.push(string.clone()),
                _ => {
                    return Err(ValidationError::single_type_error(
                        location.clone(),
                        location,
                        Location::new(),
                        item,
                        JsonType::String,
                    ))
                }
            }
        }
        Ok(Box::new(RequiredValidator { required, location }))
    }
}

impl Validate for RequiredValidator {
    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        if let Value::Object(item) = instance {
            if item.len() < self.required.len() {
                return false;
            }
            self.required
                .iter()
                .all(|property_name| item.contains_key(property_name))
        } else {
            true
        }
    }

    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        tracker: Option<&RefTracker>,
        _ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if let Value::Object(item) = instance {
            for property_name in &self.required {
                if !item.contains_key(property_name) {
                    return Err(ValidationError::required(
                        self.location.clone(),
                        crate::paths::capture_evaluation_path(tracker, &self.location),
                        location.into(),
                        instance,
                        Value::String(property_name.clone()),
                    ));
                }
            }
        }
        Ok(())
    }
    fn iter_errors<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        tracker: Option<&RefTracker>,
        _ctx: &mut ValidationContext,
    ) -> ErrorIterator<'i> {
        if let Value::Object(item) = instance {
            let mut errors = vec![];
            let eval_path = crate::paths::capture_evaluation_path(tracker, &self.location);
            for property_name in &self.required {
                if !item.contains_key(property_name) {
                    errors.push(ValidationError::required(
                        self.location.clone(),
                        eval_path.clone(),
                        location.into(),
                        instance,
                        Value::String(property_name.clone()),
                    ));
                }
            }
            if !errors.is_empty() {
                return ErrorIterator::from_iterator(errors.into_iter());
            }
        }
        no_error()
    }
}

pub(crate) struct SingleItemRequiredValidator {
    value: String,
    location: Location,
}

impl SingleItemRequiredValidator {
    #[inline]
    pub(crate) fn compile(value: &str, location: Location) -> CompilationResult<'_> {
        Ok(Box::new(SingleItemRequiredValidator {
            value: value.to_string(),
            location,
        }))
    }
}

impl DeduceType for SingleItemRequiredValidator {
    fn deduce_type(&self, type_name: &[String]) -> DeduceTypeResult<DeducedType> {
        // add required fields (type: Any -> to be refined by other validators)
        Ok(DeducedType::Struct(Box::new(StructType {
            name: type_name.to_owned(),
            attributes: HashMap::from([(
                self.value.as_str().to_owned(),
                deduced_type::StructAttribute {
                    inner_type: DeducedType::Any,
                    is_optional: false,
                },
            )]),
            additional_attributes: None,
        })))
    }
}

impl Validate for SingleItemRequiredValidator {
    fn validate<'i>(
        &self,
        instance: &'i Value,
        location: &LazyLocation,
        tracker: Option<&RefTracker>,
        ctx: &mut ValidationContext,
    ) -> Result<(), ValidationError<'i>> {
        if !self.is_valid(instance, ctx) {
            return Err(ValidationError::required(
                self.location.clone(),
                crate::paths::capture_evaluation_path(tracker, &self.location),
                location.into(),
                instance,
                Value::String(self.value.clone()),
            ));
        }
        Ok(())
    }

    fn is_valid(&self, instance: &Value, _ctx: &mut ValidationContext) -> bool {
        if let Value::Object(item) = instance {
            if item.is_empty() {
                return false;
            }
            item.contains_key(&self.value)
        } else {
            true
        }
    }
}

#[inline]
pub(crate) fn compile<'a>(
    ctx: &compiler::Context,
    _: &'a Map<String, Value>,
    schema: &'a Value,
) -> Option<CompilationResult<'a>> {
    let location = ctx.location().join("required");
    compile_with_path(schema, location)
}

#[inline]
pub(crate) fn compile_with_path(
    schema: &Value,
    location: Location,
) -> Option<CompilationResult<'_>> {
    // IMPORTANT: If this function will ever return `None`, adjust `dependencies.rs` accordingly
    match schema {
        Value::Array(items) => {
            if items.len() == 1 {
                let item = &items[0];
                if let Value::String(item) = item {
                    Some(SingleItemRequiredValidator::compile(item, location))
                } else {
                    Some(Err(ValidationError::single_type_error(
                        location.clone(),
                        location,
                        Location::new(),
                        item,
                        JsonType::String,
                    )))
                }
            } else {
                Some(RequiredValidator::compile(items, location))
            }
        }
        _ => Some(Err(ValidationError::single_type_error(
            location.clone(),
            location,
            Location::new(),
            schema,
            JsonType::Array,
        ))),
    }
}

impl DeduceType for RequiredValidator {
    fn deduce_type(&self, type_name: &[String]) -> DeduceTypeResult<DeducedType> {
        // add required fields (type: Any -> to be refined by other validators)
        Ok(DeducedType::Struct(Box::new(StructType {
            name: type_name.to_owned(),
            attributes: self
                .required
                .iter()
                .map(|name| {
                    (
                        name.clone(),
                        deduced_type::StructAttribute {
                            inner_type: DeducedType::Any,
                            is_optional: false,
                        },
                    )
                })
                .collect(),
            additional_attributes: None,
        })))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        deduced_type::{deduce_type, DeducedType, StructAttribute, StructType},
        tests_util,
    };
    use serde_json::{json, Value};
    use test_case::test_case;

    #[test_case(&json!({"required": ["a"]}), &json!({}), "/required")]
    #[test_case(&json!({"required": ["a", "b"]}), &json!({}), "/required")]
    fn location(schema: &Value, instance: &Value, expected: &str) {
        tests_util::assert_schema_location(schema, instance, expected);
    }

    #[test]
    fn deduce_type_test1() {
        assert_eq!(
            deduce_type(
                &json!({
                    "type": "object",
                    "properties": {
                        "color": {"type": "integer"},
                        "size_cm": {"type": "number"}
                    },
                    "additionalProperties": false,
                    "required": ["color", "size_cm"]
                }),
                "myname"
            ),
            Ok(DeducedType::Struct(Box::new(StructType {
                name: vec!["myname".to_owned()],
                attributes: HashMap::from([
                    (
                        "color".to_owned(),
                        StructAttribute {
                            inner_type: DeducedType::Integer,
                            is_optional: false,
                        }
                    ),
                    (
                        "size_cm".to_owned(),
                        StructAttribute {
                            inner_type: DeducedType::Number,
                            is_optional: false,
                        }
                    )
                ]),
                additional_attributes: None
            })))
        );
    }

    #[test]
    fn deduce_type_test2() {
        assert_eq!(
            deduce_type(
                &json!({
                    "type": "object",
                    "properties": {
                        "color": {"type": "integer"},
                        "size_cm": {"type": "number"}
                    },
                    "additionalProperties": false,
                    "required": ["color"]
                }),
                "myname"
            ),
            Ok(DeducedType::Struct(Box::new(StructType {
                name: vec!["myname".to_owned()],
                attributes: HashMap::from([
                    (
                        "color".to_owned(),
                        StructAttribute {
                            inner_type: DeducedType::Integer,
                            is_optional: false,
                        }
                    ),
                    (
                        "size_cm".to_owned(),
                        StructAttribute {
                            inner_type: DeducedType::Number,
                            is_optional: true,
                        }
                    )
                ]),
                additional_attributes: None
            })))
        );
    }
}
