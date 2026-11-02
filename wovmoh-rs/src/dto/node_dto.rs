use serde::{Deserialize, Serialize};

use crate::core::node::NodeId;
use crate::dto::{input_field_dto::InputFieldDto, output_pin_dto::OutputPinDto};

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeDto {
    pub id: NodeId,
    pub name: String,
    pub kind: String,
    pub position: (f64, f64),
    pub name_override: String,

    pub inputs: Vec<InputFieldDto>,
    pub outputs: Vec<OutputPinDto>,
}
