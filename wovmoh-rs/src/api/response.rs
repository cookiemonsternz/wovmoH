use serde::Serialize;

use crate::{core::graph::GraphId, dto::graph_dto::GraphDto};

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: u64,
    pub data: Result<ResponseData, String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ResponseData {
    Acknowledge,
    GraphCreated { id: GraphId },
    GraphData { graph: GraphDto },
    NodeKinds { kinds: Vec<&'static str> },
}
