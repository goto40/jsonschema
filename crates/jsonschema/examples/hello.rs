#![allow(clippy::print_stdout)]
use serde_json::{self, json};

fn main() {
    let schema = serde_json::from_str(
        r#"
        {
            "type": "object",
            "properties": {
                "color": {"enum": ["red", "green", "yellow"]},
                "size_cm": {"type": "number"}
            },
            "additionalProperties": false,
            "required": ["color", "size_cm"]
        }
        "#,
    )
    .unwrap();
    let validator = jsonschema::validator_for(&schema).unwrap();
    //println!("Hello {validator:?}");
    let instance = json!({"color": "red", "size_cm": 5.2});
    let evaluation = validator.evaluate(&instance);
    //println!("evaluation {evaluation:?}");
}
