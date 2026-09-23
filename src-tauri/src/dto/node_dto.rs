use serde::Serialize;

use crate::core::node::NodeId;
use crate::dto::{input_field_dto::InputFieldDto, output_pin_dto::OutputPinDto};

#[derive(Serialize)]
pub struct NodeDto {
    pub id: NodeId,
    pub name: &'static str,
    pub kind: &'static str,
    pub position: (f64, f64),

    pub inputs: Vec<InputFieldDto>,
    pub outputs: Vec<OutputPinDto>,
}
