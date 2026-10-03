use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use strum::IntoEnumIterator;

use crate::core::node::{NodeKind, NodeUIState};
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
                    name_override: "Name override".to_string(),
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
        Command::Connect {
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
    }
}
