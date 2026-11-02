use serde::{Deserialize, Serialize};

use crate::core::output_pin::OutputId;
use crate::types::data_type::{DataType, DataValue};

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputPinDto {
    pub name: String,
    pub data_type: DataType,
    pub value: DataValue,
}
