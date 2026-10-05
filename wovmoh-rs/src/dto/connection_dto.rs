use serde::Serialize;

use crate::core::node::NodeId;

#[derive(Debug, Serialize)]
pub struct ConnectionDto {
    pub from_node: NodeId,
    pub from_index: usize,
    pub to_node: NodeId,
    pub to_index: usize,
}
