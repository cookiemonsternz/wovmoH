use serde::Serialize;

use crate::core::node::InputConstraints;
use crate::types::data_type::{DataType, DataValue};

#[derive(Debug, Serialize)]
pub struct InputFieldDto {
    pub name: &'static str,
    pub data_type: DataType,
    pub value: DataValue,
    pub constraints: InputConstraints,
}
