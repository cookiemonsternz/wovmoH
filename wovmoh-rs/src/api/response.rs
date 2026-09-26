use serde::{Deserialize, Serialize};

use crate::core::graph::GraphId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub id: u64,
    pub data: Result<ResponseData, String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ResponseData {
    Acknowledge,
    GraphCreated { id: GraphId },
}
