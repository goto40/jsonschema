#![allow(clippy::print_stdout)]
use serde_json::json;

fn main() {
    let schema = json!(
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
        "#
    );
    let validator = jsonschema::validator_for(&schema).unwrap();
    //println!("Hello {validator:?}");
    let instance = json!({"color": "red", "size_cm": 5.2});
    let evaluation = validator.evaluate(&instance);
    //println!("evaluation {evaluation:?}");
}
