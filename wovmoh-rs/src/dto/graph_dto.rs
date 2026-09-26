use serde::Serialize;

use crate::core::graph::GraphId;

use super::connection_dto::ConnectionDto;
use super::node_dto::NodeDto;

#[derive(Serialize)]
pub struct GraphDto {
    pub id: GraphId,
    pub nodes: Vec<NodeDto>,
    pub connections: Vec<ConnectionDto>,
}
