use serde::Deserialize;

use crate::{
    core::{
        graph::{Graph, GraphId},
        node::NodeId,
    },
    types::data_type::DataValue,
};

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: u64,
    pub command: Command,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    Poll,
    AddExampleGraph {
        id: GraphId,
    },
    AddGraph,
    GetGraph {
        id: GraphId,
    },
    GetNodeKinds,
    SetInputFieldValue {
        graph_id: GraphId,
        node_id: NodeId,
        field_index: usize,
        value: DataValue,
    },
    Connect {
        graph_id: GraphId,
        node_from: NodeId,
        pin_from: usize,
        node_to: NodeId,
        field_to: usize,
    },
}
