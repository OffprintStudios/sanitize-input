use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Delta {
    ops: Vec<DeltaOps>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeltaOps {
    attributes: Option<Value>,
    insert: Value
}