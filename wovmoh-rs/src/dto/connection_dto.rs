use serde::{Deserialize, Serialize};

use crate::core::node::NodeId;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionDto {
    pub from_node: NodeId,
    pub from_index: usize,
    pub to_node: NodeId,
    pub to_index: usize,
}
