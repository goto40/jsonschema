#![allow(clippy::print_stdout)]
use jsonschema::Validator;
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
    let tst = Validator::options().build(&schema).unwrap();
    println!("Hello {:#?}", tst.get_node());
    let instance = json!({"color": "red", "size_cm": 5.2});
    let _evaluation = validator.evaluate(&instance);
    //println!("evaluation {evaluation:?}");
}
