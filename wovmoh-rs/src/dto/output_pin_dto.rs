use serde::Serialize;

use crate::core::output_pin::OutputId;
use crate::types::data_type::{DataType, DataValue};

#[derive(Debug, Serialize)]
pub struct OutputPinDto {
    pub name: &'static str,
    pub data_type: DataType,
    pub value: DataValue,
}
