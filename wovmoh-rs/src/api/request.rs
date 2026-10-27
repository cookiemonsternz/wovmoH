use serde::Deserialize;

use crate::{
    core::{
        graph::{Graph, GraphId},
        node::{NodeId, NodeKind},
    },
    io::midi::MidiDataMessage,
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
    GetNodeDescriptors,
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
    ConnectMidiInput {
        id: String,
    },
    ConnectMidiOutput {
        id: String,
    },
    GetConnectedMidiInputs,
    GetConnectedMidiOutputs,
    SubscribeMidiInputToField {
        graph_id: GraphId,
        node_id: NodeId,
        field_index: usize,
        message: MidiDataMessage,
    },
    GetMidiInputSubscriptions,
    UnsubscribeMidiInputFromField {
        graph_id: GraphId,
        node_id: NodeId,
        field_index: usize,
        message: MidiDataMessage,
    },
    SubscribeMidiOutputToPin {
        graph_id: GraphId,
        node_id: NodeId,
        pin_index: usize,
        midi_output: String,
        message: MidiDataMessage,
    },
    GetMidiOutputSubscriptions,
    UnsubscribeMidiOutputFromPin {
        graph_id: GraphId,
        node_id: NodeId,
        pin_index: usize,
        midi_output: String,
        message: MidiDataMessage,
    },
    GetMidiInputNameFromId {
        id: String,
    },
    GetMidiOutputNameFromId {
        id: String,
    },
    ArtNetPoll,
    SubscribeArtnetInputToField {
        graph_id: GraphId,
        node_id: NodeId,
        field_index: usize,
        channel: usize,
    },
    UnsubscribeArtnetInputFromField {
        graph_id: GraphId,
        node_id: NodeId,
        field_index: usize,
        channel: usize,
    },
    GetArtnetInputSubscriptions,
    SubscribeArtnetOutputToPin {
        graph_id: GraphId,
        node_id: NodeId,
        pin_index: usize,
        channel: usize,
    },
    UnsubscribeArtnetOutputFromPin {
        graph_id: GraphId,
        node_id: NodeId,
        pin_index: usize,
        channel: usize,
    },
    GetArtnetOutputSubscriptions,
}
