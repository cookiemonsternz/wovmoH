use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

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
        Command::AddGraph => {
            let id = app.lock().as_mut().unwrap().graphs.add_graph();
            write(ResponseData::GraphCreated { id }, request, stream);
        }
        Command::GetGraph => todo!(),
    }
}
