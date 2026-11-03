use serde::{Deserialize, Serialize};

use crate::types::data_type::{DataType, DataValue};

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputPinDto {
    pub name: String,
    pub data_type: DataType,
    pub value: DataValue,
}
