use serde::Serialize;

use crate::core::input_field::InputId;
use crate::types::data_type::{DataType, DataValue};

#[derive(Serialize)]
pub struct InputFieldDto {
    pub name: &'static str,
    pub data_type: DataType,
    pub value: DataValue,
}
