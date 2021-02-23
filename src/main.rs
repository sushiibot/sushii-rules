use schemars::{schema_for, JsonSchema};

mod model;

fn main() {
    let schema = schema_for!(model::Rule);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}