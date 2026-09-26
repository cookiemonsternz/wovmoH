use serde::Deserialize;

use crate::core::graph::GraphId;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: u64,
    pub command: Command,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    Poll,
    AddExampleGraph { id: GraphId },
    AddGraph,
    GetGraph { id: GraphId },
    GetNodeKinds,
}
