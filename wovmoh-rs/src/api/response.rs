use serde::Serialize;

use crate::{
    core::{graph::GraphId, node::NodeDescriptorDTO},
    dto::graph_dto::GraphDto,
    io::{
        artnet::{ArtnetInputFieldSubscriptionDTO, ArtnetOutputPinSubscriptionDTO},
        midi::{
            MidiInputFieldSubscription, MidiInputFieldSubscriptionDTO,
            MidiOutputPinSubscriptionDTO, PortDescriptor,
        },
    },
};

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: u64,
    pub data: Result<ResponseData, String>,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ResponseData {
    Acknowledge,
    GraphCreated {
        id: GraphId,
    },
    GraphData {
        graph: GraphDto,
    },
    NodeDescriptors {
        descriptors: Vec<NodeDescriptorDTO>,
    },
    AvailableMidiInputs {
        inputs: Vec<PortDescriptor>,
    },
    AvailableMidiOutputs {
        outputs: Vec<PortDescriptor>,
    },
    ConnectedMidiInputs {
        inputs: Vec<PortDescriptor>,
    },
    ConnectedMidiOutputs {
        outputs: Vec<PortDescriptor>,
    },
    MidiInputSubscriptions {
        subscriptions: Vec<MidiInputFieldSubscriptionDTO>,
    },
    MidiOutputSubscriptions {
        subscriptions: Vec<MidiOutputPinSubscriptionDTO>,
    },
    MidiInputName {
        name: String,
    },
    MidiOutputName {
        name: String,
    },
    ArtnetInputSubscriptions {
        subscriptions: Vec<ArtnetInputFieldSubscriptionDTO>,
    },
    ArtnetOutputSubscriptions {
        subscriptions: Vec<ArtnetOutputPinSubscriptionDTO>,
    },
}
