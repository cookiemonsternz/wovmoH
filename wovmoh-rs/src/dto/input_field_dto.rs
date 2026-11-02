use serde::{Deserialize, Serialize};

use crate::core::node::InputConstraints;
use crate::types::data_type::{DataType, DataValue};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputFieldDto {
    pub name: String,
    pub data_type: DataType,
    pub value: DataValue,
    pub constraints: InputConstraints,
}
