use std::io::Write;
use std::net::TcpStream;
use std::path::PathBuf;
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

fn write_error(message: impl Into<String>, request: Request, stream: &mut TcpStream) {
    write(
        ResponseData::Error {
            message: message.into(),
        },
        request,
        stream,
    );
}

pub fn handle_request(app: Arc<Mutex<App>>, request: Request, stream: &mut TcpStream) {
    let mut app = match app.lock() {
        Ok(app) => app,
        Err(_) => {
            write_error("App mutex locket", request, stream);
            return;
        }
    };
    match request.command {
        Command::Poll => write(ResponseData::Acknowledge, request, stream),
        Command::AddExampleGraph { id } => {
            let graph = match app.graphs.try_get_graph_mut(id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };

            graph.add_node(NodeKind::ConstantColor, NodeUIState::default());
            graph.add_node(NodeKind::ConstantColor, NodeUIState::default());
            if let Err(error) = graph.try_connect(0, 1) {
                write_error(error, request, stream);
                return;
            }
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
            let id = app.graphs.add_graph();
            write(ResponseData::GraphCreated { id }, request, stream);
        }
        Command::GetGraph { id } => {
            let graph = match app.graphs.try_get_graph(id) {
                Ok(graph) => graph.to_dto(),
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
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
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let field = match graph.try_input_for_mut(node_id, field_index) {
                Ok(field) => field,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
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
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let pin_id = match graph.try_output_id_for(node_from, pin_from) {
                Ok(pin_id) => pin_id,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let field_id = match graph.try_input_id_for(node_to, field_to) {
                Ok(field_id) => field_id,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            if let Err(error) = graph.try_connect(pin_id, field_id) {
                write_error(error, request, stream);
                return;
            }
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::DisconnectNodes {
            graph_id,
            node_from: _,
            pin_from: _,
            node_to,
            field_to,
        } => {
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let field_id = match graph.try_input_id_for(node_to, field_to) {
                Ok(field_id) => field_id,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            if let Err(error) = graph.try_disconnect(field_id) {
                write_error(error, request, stream);
                return;
            }
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::AddNode {
            graph_id,
            kind,
            position,
        } => {
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
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
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let node = match graph.try_get_node_mut(node_id) {
                Ok(node) => node,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            node.ui_state.position = position;
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::DeleteNode { graph_id, node_id } => {
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            if let Err(error) = graph.try_remove_node(node_id) {
                write_error(error, request, stream);
                return;
            }
            write(ResponseData::Acknowledge, request, stream);
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
            match app.midi.connect_input(id.to_string()) {
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
                _ => {}
            };
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::ConnectMidiOutput { ref id } => {
            match app.midi.connect_output(id.to_string()) {
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
                _ => {}
            };
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetConnectedMidiInputs => {
            let inputs = match app.midi.get_connected_inputs() {
                Ok(inputs) => inputs,
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
            };
            write(
                ResponseData::ConnectedMidiInputs { inputs },
                request,
                stream,
            );
        }
        Command::GetConnectedMidiOutputs => {
            let outputs = match app.midi.get_connected_outputs() {
                Ok(outputs) => outputs,
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
            };
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
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let field_id = graph.input_id_for(node_id, field_index);
            app.midi.subscribe_input(MidiInputFieldSubscription {
                graph: graph_id,
                field: field_id,
                message: message.clone(),
            });
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetMidiInputSubscriptions => {
            let mut subscriptions: Vec<MidiInputFieldSubscriptionDTO> = Vec::new();

            let current_subscriptions = match app.midi.input_subscriptions.read() {
                Ok(subscriptions) => subscriptions,
                Err(_) => {
                    write_error("App mutex locked", request, stream);
                    return;
                }
            };

            for subscription in current_subscriptions.iter() {
                let graph = match app.graphs.try_get_graph(subscription.graph) {
                    Ok(graph) => graph,
                    Err(err) => {
                        write_error(err, request, stream);
                        return;
                    }
                };
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
            let graph = match app.graphs.try_get_graph(graph_id) {
                Ok(graph) => graph,
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
            };
            let field_id = graph.input_id_for(node_id, field_index);
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
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
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
            let mut subscriptions: Vec<MidiOutputPinSubscriptionDTO> = Vec::new();
            for subscription in app.midi.output_subscriptions.read().unwrap().iter() {
                let graph = match app.graphs.try_get_graph(subscription.graph) {
                    Ok(graph) => graph,
                    Err(error) => {
                        write_error(error, request, stream);
                        return;
                    }
                };
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
            let graph = match app.graphs.try_get_graph(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let pin_id = graph.output_id_for(node_id, pin_index);
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
                    name: match MidiManager::get_input_name_from_id(id.clone()) {
                        Ok(name) => name,
                        Err(err) => {
                            write_error(err, request, stream);
                            return;
                        }
                    },
                },
                request,
                stream,
            );
        }
        Command::GetMidiOutputNameFromId { ref id } => {
            write(
                ResponseData::MidiOutputName {
                    name: match MidiManager::get_output_name_from_id(id.clone()) {
                        Ok(name) => name,
                        Err(err) => {
                            write_error(err, request, stream);
                            return;
                        }
                    },
                },
                request,
                stream,
            );
        }
        Command::ArtNetPoll => {
            app.artnet.poll();
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::SubscribeArtnetInputToField {
            graph_id,
            node_id,
            field_index,
            channel,
        } => {
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let field_id = graph.input_id_for(node_id, field_index);
            match app.artnet.subscribe_input(ArtnetInputFieldSubscription {
                graph_id,
                field: field_id,
                channel: channel,
            }) {
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
                _ => {}
            };
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::UnsubscribeArtnetInputFromField {
            graph_id,
            node_id,
            field_index,
            channel,
        } => {
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let field_id = graph.input_id_for(node_id, field_index);
            match app.artnet.unsubscribe_input(ArtnetInputFieldSubscription {
                graph_id,
                field: field_id,
                channel: channel,
            }) {
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
                _ => {}
            };
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetArtnetInputSubscriptions => {
            let mut subscriptions: Vec<ArtnetInputFieldSubscriptionDTO> = Vec::new();
            for subscription in app.artnet.input_subscriptions.read().unwrap().iter() {
                let graph = match app.graphs.try_get_graph(subscription.graph_id) {
                    Ok(graph) => graph,
                    Err(error) => {
                        write_error(error, request, stream);
                        return;
                    }
                };
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
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let pin_id = graph.output_id_for(node_id, pin_index);
            match app.artnet.subscribe_output(ArtnetOutputPinSubscription {
                graph_id,
                pin: pin_id,
                channel: channel,
            }) {
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
                _ => {}
            };
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::UnsubscribeArtnetOutputFromPin {
            graph_id,
            node_id,
            pin_index,
            channel,
        } => {
            let graph = match app.graphs.try_get_graph_mut(graph_id) {
                Ok(graph) => graph,
                Err(error) => {
                    write_error(error, request, stream);
                    return;
                }
            };
            let pin_id = graph.output_id_for(node_id, pin_index);
            match app.artnet.unsubscribe_output(ArtnetOutputPinSubscription {
                graph_id,
                pin: pin_id,
                channel: channel,
            }) {
                Err(err) => {
                    write_error(err, request, stream);
                    return;
                }
                _ => {}
            };
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::GetArtnetOutputSubscriptions => {
            let mut subscriptions: Vec<ArtnetOutputPinSubscriptionDTO> = Vec::new();
            for subscription in app.artnet.output_subscriptions.read().unwrap().iter() {
                let graph = match app.graphs.try_get_graph(subscription.graph_id) {
                    Ok(graph) => graph,
                    Err(error) => {
                        write_error(error, request, stream);
                        return;
                    }
                };
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
        Command::SaveFile { ref path } => {
            let path = PathBuf::from(path);
            if let Err(error) = app.save(path) {
                write_error(error, request, stream);
                return;
            }
            write(ResponseData::Acknowledge, request, stream);
        }
        Command::LoadFile { ref path } => {
            let path = PathBuf::from(path);
            if let Err(error) = app.load_override(path) {
                write_error(error, request, stream);
                return;
            }
            write(ResponseData::Acknowledge, request, stream);
        }
    }
}
