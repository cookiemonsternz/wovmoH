use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use strum::IntoEnumIterator;

use crate::core::node::{NodeDescriptorDTO, NodeKind, NodeUIState};
use crate::io::artnet::{
    ArtnetInputFieldSubscription, ArtnetInputFieldSubscriptionDTO, ArtnetOutputPinSubscription,
    ArtnetOutputPinSubscriptionDTO,
};
use crate::io::midi::{
    MidiInputFieldSubscription, MidiInputFieldSubscriptionDTO, MidiManager,
    MidiOutputPinSubscription, MidiOutputPinSubscriptionDTO,
};
use crate::{
    api::{
        request::{Command, Request},
        response::{Response, ResponseData},
    },
    app::App,
};

fn write(data: ResponseData, request: Request, stream: &mut TcpStream) {
    let response = Response {
        id: request.id,
        data: Ok(data),
    };

    let mut json = serde_json::to_string(&response).unwrap();
    json.push('\n');

    stream.write(json.as_bytes()).unwrap();
}

pub fn handle_request(app: Arc<Mutex<App>>, request: Request, stream: &mut TcpStream) {
    match request.command {
        Command::Poll => write(ResponseData::Acknowledge, request, stream),
        Command::AddExampleGraph { id } => {
            let mut app = app.lock().unwrap();
            let graph = app.graphs.get_graph_mut(id);

            graph.add_node(NodeKind::ConstantColor, NodeUIState::default());
            graph.add_node(NodeKind::ConstantColor, NodeUIState::default());
            graph.connect(0, 1);
            graph.add_node(NodeKind::ConstantNumber, NodeUIState::default());
            graph.add_node(
                NodeKind::ConstantBoolean,
                NodeUIState {
                    position: (5.0, 1.0),
                    name_override: "Test Name override!".to_string(),
                },
            );
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::AddGraph => {
            let id = app.lock().as_mut().unwrap().graphs.add_graph();
            write(ResponseData::GraphCreated { id }, request, stream);
        }
        Command::GetGraph { id } => {
            let graph = app.lock().unwrap().graphs.get_graph(id).to_dto();
            write(ResponseData::GraphData { graph }, request, stream);
        }
        Command::GetNodeDescriptors => {
            let descriptors: Vec<NodeDescriptorDTO> = NodeKind::iter()
                .map(|x| -> NodeDescriptorDTO {
                    NodeDescriptorDTO {
                        kind: x.into(),
                        name: x.descriptor().name,
                        category: x.descriptor().category,
                    }
                })
                .collect();

            write(
                ResponseData::NodeDescriptors { descriptors },
                request,
                stream,
            );
        }
        Command::SetInputFieldValue {
            graph_id,
            node_id,
            field_index,
            value,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let field = graph.input_for_mut(node_id, field_index);
            field.value = value;

            write(ResponseData::Acknowledge, request, stream);
        }
        Command::ConnectNodes {
            graph_id,
            node_from,
            pin_from,
            node_to,
            field_to,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let pin_id = graph.output_id_for(node_from, pin_from);
            let field_id = graph.input_id_for(node_to, field_to);
            graph.connect(pin_id, field_id);
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::DisconnectNodes {
            graph_id,
            node_from,
            pin_from,
            node_to,
            field_to,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let field_id = graph.input_id_for(node_to, field_to);
            graph.disconnect(field_id);
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::AddNode {
            graph_id,
            kind,
            position,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            graph.add_node(
                kind,
                NodeUIState {
                    position: position,
                    name_override: String::new(),
                },
            );
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::SetNodePosition {
            graph_id,
            node_id,
            position,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let node = graph.get_node_mut(node_id);
            node.ui_state.position = position;
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::DeleteNode { graph_id, node_id } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            graph.remove_node(node_id);
        }
        Command::GetAvailableMidiInputs => {
            write(
                ResponseData::AvailableMidiInputs {
                    inputs: MidiManager::get_available_input_ports(),
                },
                request,
                stream,
            );
        }
        Command::GetAvailableMidiOutputs => {
            write(
                ResponseData::AvailableMidiOutputs {
                    outputs: MidiManager::get_available_output_ports(),
                },
                request,
                stream,
            );
        }
        Command::ConnectMidiInput { ref id } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            app.midi.connect_input(id.to_string());
            drop(lock);
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::ConnectMidiOutput { ref id } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            app.midi.connect_output(id.to_string());
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetConnectedMidiInputs => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let inputs = app.midi.get_connected_inputs();
            write(
                ResponseData::ConnectedMidiInputs { inputs },
                request,
                stream,
            );
        }
        Command::GetConnectedMidiOutputs => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let outputs = app.midi.get_connected_outputs();
            write(
                ResponseData::ConnectedMidiOutputs { outputs },
                request,
                stream,
            );
        }
        Command::SubscribeMidiInputToField {
            graph_id,
            node_id,
            field_index,
            ref message,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let field_id = graph.input_id_for(node_id, field_index);
            app.midi.subscribe_input(MidiInputFieldSubscription {
                graph: graph_id,
                field: field_id,
                message: message.clone(),
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetMidiInputSubscriptions => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let mut subscriptions: Vec<MidiInputFieldSubscriptionDTO> = Vec::new();
            for subscription in app.midi.input_subscriptions.read().unwrap().iter() {
                let graph = app.graphs.get_graph(subscription.graph);
                let node = graph.node_for_input(subscription.field);
                let index = graph.input_index_for_input(subscription.field);
                subscriptions.push(MidiInputFieldSubscriptionDTO {
                    graph_id: subscription.graph,
                    node_id: node,
                    field_index: index,
                    message: subscription.message.clone(),
                });
            }

            write(
                ResponseData::MidiInputSubscriptions { subscriptions },
                request,
                stream,
            );
        }
        Command::UnsubscribeMidiInputFromField {
            graph_id,
            node_id,
            field_index,
            ref message,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let field_id = app
                .graphs
                .get_graph(graph_id)
                .input_id_for(node_id, field_index);
            app.midi.unsubscribe_input(MidiInputFieldSubscription {
                graph: graph_id,
                field: field_id,
                message: message.clone(),
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::SubscribeMidiOutputToPin {
            graph_id,
            node_id,
            pin_index,
            ref midi_output,
            ref message,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let pin_id = graph.output_id_for(node_id, pin_index);
            app.midi.subscribe_output(MidiOutputPinSubscription {
                midi_output: midi_output.clone(),
                graph: graph_id,
                pin: pin_id,
                message: message.clone(),
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetMidiOutputSubscriptions => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let mut subscriptions: Vec<MidiOutputPinSubscriptionDTO> = Vec::new();
            for subscription in app.midi.output_subscriptions.read().unwrap().iter() {
                let graph = app.graphs.get_graph(subscription.graph);
                let node = graph.node_for_output(subscription.pin);
                let index = graph.output_index_for_output(subscription.pin);
                subscriptions.push(MidiOutputPinSubscriptionDTO {
                    graph_id: subscription.graph,
                    node_id: node,
                    pin_index: index,
                    midi_output: subscription.midi_output.clone(),
                    message: subscription.message.clone(),
                });
            }

            write(
                ResponseData::MidiOutputSubscriptions { subscriptions },
                request,
                stream,
            );
        }
        Command::UnsubscribeMidiOutputFromPin {
            graph_id,
            node_id,
            pin_index,
            ref midi_output,
            ref message,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let pin_id = app
                .graphs
                .get_graph(graph_id)
                .output_id_for(node_id, pin_index);
            app.midi.unsubscribe_output(MidiOutputPinSubscription {
                midi_output: midi_output.clone(),
                graph: graph_id,
                pin: pin_id,
                message: message.clone(),
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetMidiInputNameFromId { ref id } => {
            write(
                ResponseData::MidiInputName {
                    name: MidiManager::get_input_name_from_id(id.clone()),
                },
                request,
                stream,
            );
        }
        Command::GetMidiOutputNameFromId { ref id } => {
            write(
                ResponseData::MidiOutputName {
                    name: MidiManager::get_output_name_from_id(id.clone()),
                },
                request,
                stream,
            );
        }
        Command::ArtNetPoll => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            app.artnet.poll();
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::SubscribeArtnetInputToField {
            graph_id,
            node_id,
            field_index,
            channel,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let field_id = graph.input_id_for(node_id, field_index);
            app.artnet.subscribe_input(ArtnetInputFieldSubscription {
                graph_id,
                field: field_id,
                channel: channel,
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::UnsubscribeArtnetInputFromField {
            graph_id,
            node_id,
            field_index,
            channel,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let field_id = graph.input_id_for(node_id, field_index);
            app.artnet.unsubscribe_input(ArtnetInputFieldSubscription {
                graph_id,
                field: field_id,
                channel: channel,
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetArtnetInputSubscriptions => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let mut subscriptions: Vec<ArtnetInputFieldSubscriptionDTO> = Vec::new();
            for subscription in app.artnet.input_subscriptions.read().unwrap().iter() {
                let graph = app.graphs.get_graph(subscription.graph_id);
                let node = graph.node_for_input(subscription.field);
                let index = graph.input_index_for_input(subscription.field);
                subscriptions.push(ArtnetInputFieldSubscriptionDTO {
                    graph_id: subscription.graph_id,
                    node_id: node,
                    field_index: index,
                    channel: subscription.channel,
                });
            }

            write(
                ResponseData::ArtnetInputSubscriptions { subscriptions },
                request,
                stream,
            );
        }
        Command::SubscribeArtnetOutputToPin {
            graph_id,
            node_id,
            pin_index,
            channel,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let pin_id = graph.output_id_for(node_id, pin_index);
            app.artnet.subscribe_output(ArtnetOutputPinSubscription {
                graph_id,
                pin: pin_id,
                channel: channel,
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::UnsubscribeArtnetOutputFromPin {
            graph_id,
            node_id,
            pin_index,
            channel,
        } => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let graph = app.graphs.get_graph_mut(graph_id);
            let pin_id = graph.output_id_for(node_id, pin_index);
            app.artnet.unsubscribe_output(ArtnetOutputPinSubscription {
                graph_id,
                pin: pin_id,
                channel: channel,
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetArtnetOutputSubscriptions => {
            let mut lock = app.lock();
            let app = lock.as_mut().unwrap();
            let mut subscriptions: Vec<ArtnetOutputPinSubscriptionDTO> = Vec::new();
            for subscription in app.artnet.output_subscriptions.read().unwrap().iter() {
                let graph = app.graphs.get_graph(subscription.graph_id);
                let node = graph.node_for_input(subscription.pin);
                let index = graph.output_index_for_output(subscription.pin);
                subscriptions.push(ArtnetOutputPinSubscriptionDTO {
                    graph_id: subscription.graph_id,
                    node_id: node,
                    pin_index: index,
                    channel: subscription.channel,
                });
            }

            write(
                ResponseData::ArtnetOutputSubscriptions { subscriptions },
                request,
                stream,
            );
        }
    }
}
