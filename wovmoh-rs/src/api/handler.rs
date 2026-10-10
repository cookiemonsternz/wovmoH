use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use strum::IntoEnumIterator;

use crate::core::node::{NodeKind, NodeUIState};
use crate::io::midi::{MidiInputFieldSubscriptionDTO, MidiManager};
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
        Command::GetNodeKinds => {
            let kinds: Vec<&'static str> = NodeKind::iter().map(|x| x.into()).collect();
            write(ResponseData::NodeKinds { kinds }, request, stream);
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
            app.midi
                .subscribe(crate::io::midi::MidiInputFieldSubscription {
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
    }
}
