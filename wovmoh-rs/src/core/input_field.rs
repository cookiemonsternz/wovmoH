use crate::{
    core::{node::NodeId, output_pin::OutputId},
    types::data_type::DataValue,
};

pub type InputId = usize;

pub struct InputField {
    pub parent: NodeId,
    pub index: u8,
    pub value: DataValue,
    pub connected_output: Option<OutputId>,
}
