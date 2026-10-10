use serde::Deserialize;

use crate::{
    core::{
        graph::{Graph, GraphId},
        node::{NodeId, NodeKind},
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
    ConnectNodes {
        graph_id: GraphId,
        node_from: NodeId,
        pin_from: usize,
        node_to: NodeId,
        field_to: usize,
    },
    DisconnectNodes {
        graph_id: GraphId,
        node_from: NodeId,
        pin_from: usize,
        node_to: NodeId,
        field_to: usize,
    },
    AddNode {
        graph_id: GraphId,
        kind: NodeKind,
        position: (f64, f64),
    },
    SetNodePosition {
        graph_id: GraphId,
        node_id: NodeId,
        position: (f64, f64),
    },
    DeleteNode {
        graph_id: GraphId,
        node_id: NodeId,
    },
    GetAvailableMidiInputs,
    GetAvailableMidiOutputs,
}
