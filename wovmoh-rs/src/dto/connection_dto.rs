use serde::Serialize;

use crate::core::{input_field::InputId, output_pin::OutputId};

#[derive(Serialize)]
pub struct ConnectionDto {
    pub from: InputId,
    pub to: OutputId,
}
