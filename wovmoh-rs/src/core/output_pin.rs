use crate::{
    core::{input_field::InputId, node::NodeId},
    types::data_type::DataValue,
};

pub type OutputId = usize;

pub struct OutputPin {
    pub parent: NodeId,
    pub index: usize,
    pub value: DataValue,
    pub connections: Vec<InputId>,
}
