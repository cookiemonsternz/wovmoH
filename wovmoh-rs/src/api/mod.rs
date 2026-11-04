pub mod handler;
pub mod request;
pub mod response;

#[cfg(test)]
mod tests {
    use std::{
        io::{BufRead, BufReader, Write},
        net::TcpStream,
        sync::{Arc, Mutex, mpsc},
        thread::JoinHandle,
    };

    use crate::{app::App, transport::tcp};

    pub struct Test {
        join_handle: Option<JoinHandle<()>>,
        shutdown_tx: mpsc::Sender<()>,
        stream: TcpStream,
    }

    impl Test {
        pub fn new() -> Test {
            let app = Arc::new(Mutex::new(App::new()));
            let (join_handle, shutdown_tx, addr) =
                tcp::run(app.clone(), "127.0.0.1:0".parse().unwrap());
            let stream = TcpStream::connect(addr).unwrap();

            Test {
                join_handle: Some(join_handle),
                shutdown_tx,
                stream,
            }
        }

        pub fn call(&mut self, request: serde_json::Value) -> serde_json::Value {
            let mut request = serde_json::to_string(&request).unwrap();
            request.push('\n');

            self.stream.write_all(request.as_bytes()).unwrap();
            self.stream.flush().unwrap();

            let mut response = String::new();
            BufReader::new(self.stream.try_clone().unwrap())
                .read_line(&mut response)
                .unwrap();

            serde_json::from_str(&response).unwrap()
        }
    }

    impl Drop for Test {
        fn drop(&mut self) {
            let _ = self.stream.shutdown(std::net::Shutdown::Both);
            let _ = self.shutdown_tx.send(());

            if let Some(handle) = self.join_handle.take() {
                handle.join().unwrap();
            }
        }
    }

    #[test]
    fn poll() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "Poll"
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn add_graph() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddGraph"
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "GraphCreated");
        assert_eq!(response["data"]["Ok"]["id"], 1);
    }

    #[test]
    fn get_graph() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetGraph",
                "id": 0
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "GraphData");
        assert_eq!(response["data"]["Ok"]["graph"]["id"], 0);
    }

    #[test]
    fn get_graph_bad() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetGraph",
                "id": 1
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_node_descriptors() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetNodeDescriptors",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "NodeDescriptors");
    }

    #[test]
    fn add_node() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn add_node_bad() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 1,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "BAD NODE KIND",
                "position": [0, 0],
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn set_input_field_value() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "SetInputFieldValue",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "value": {
                    "type": "Number",
                    "value": 32.0
                }
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn set_input_field_value_bad() {
        let mut test = Test::new();

        // Bad Graph

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SetInputFieldValue",
                "graph_id": 1,
                "node_id": 0,
                "field_index": 0,
                "value": {
                    "type": "Number",
                    "value": 32.0
                }
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node

        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "SetInputFieldValue",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "value": {
                    "type": "Number",
                    "value": 32.0
                }
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad field
        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "SetInputFieldValue",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 1,
                "value": {
                    "type": "Number",
                    "value": 32.0
                }
            }
        }));

        assert_eq!(response["id"], 3);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad value
        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "SetInputFieldValue",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 1,
                "value": {
                    "type": "Boolean",
                    "value": false
                }
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn connect_nodes() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "ConnectNodes",
                "graph_id": 0,
                "node_from": 0,
                "pin_from": 0,
                "node_to": 1,
                "field_to": 0,
            }
        }));

        assert_eq!(response["id"], 3);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn connect_nodes_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "ConnectNodes",
                "graph_id": 1,
                "node_from": 0,
                "pin_from": 0,
                "node_to": 1,
                "field_to": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad nodes
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "ConnectNodes",
                "graph_id": 0,
                "node_from": 0,
                "pin_from": 0,
                "node_to": 1,
                "field_to": 0,
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad field / pin
        test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 5,
            "command": {
                "type": "ConnectNodes",
                "graph_id": 0,
                "node_from": 0,
                "pin_from": 1,
                "node_to": 1,
                "field_to": 1,
            }
        }));

        assert_eq!(response["id"], 5);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn disconnect_nodes() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "ConnectNodes",
                "graph_id": 0,
                "node_from": 0,
                "pin_from": 0,
                "node_to": 1,
                "field_to": 0,
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "DisconnectNodes",
                "graph_id": 0,
                "node_from": 0,
                "pin_from": 0,
                "node_to": 1,
                "field_to": 0,
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn disconnect_nodes_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "DisconnectNodes",
                "graph_id": 1,
                "node_from": 0,
                "pin_from": 0,
                "node_to": 1,
                "field_to": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad nodes
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "DisconnectNodes",
                "graph_id": 0,
                "node_from": 0,
                "pin_from": 0,
                "node_to": 1,
                "field_to": 0,
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // No connection
        test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 5,
            "command": {
                "type": "DisconnectNodes",
                "graph_id": 0,
                "node_from": 0,
                "pin_from": 0,
                "node_to": 1,
                "field_to": 0,
            }
        }));

        assert_eq!(response["id"], 5);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn set_node_position() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "SetNodePosition",
                "graph_id": 0,
                "node_id": 0,
                "position": [10, 10],
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn set_node_position_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SetNodePosition",
                "graph_id": 1,
                "node_id": 0,
                "position": [10, 10],
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "SetNodePosition",
                "graph_id": 1,
                "node_id": 1,
                "position": [10, 10],
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn delete_node() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "DeleteNode",
                "graph_id": 0,
                "node_id": 0,
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn delete_node_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "DeleteNode",
                "graph_id": 1,
                "node_id": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "DeleteNode",
                "graph_id": 0,
                "node_id": 0,
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_available_midi_inputs() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetAvailableMidiInputs",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "AvailableMidiInputs");
    }

    #[test]
    fn get_available_midi_outputs() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetAvailableMidiOutputs",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "AvailableMidiOutputs");
    }

    #[test]
    fn connect_midi_input() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "ConnectMidiInput",
                "id": "test-input",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn connect_midi_output() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "ConnectMidiOutput",
                "id": "test-output",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_connected_midi_inputs() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetConnectedMidiInputs",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "ConnectedMidiInputs");
    }

    #[test]
    fn get_connected_midi_outputs() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetConnectedMidiOutputs",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "ConnectedMidiOutputs");
    }

    #[test]
    fn subscribe_midi_input_to_field() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeMidiInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn subscribe_midi_input_to_field_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeMidiInputToField",
                "graph_id": 1,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "SubscribeMidiInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad field index
        test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "SubscribeMidiInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 1,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad midi message
        let response = test.call(serde_json::json!({
            "id": 5,
            "command": {
                "type": "SubscribeMidiInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "BAD MIDI MESSAGE",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 5);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_midi_input_subscriptions() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetMidiInputSubscriptions",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "MidiInputSubscriptions");
    }

    #[test]
    fn unsubscribe_midi_input_from_field() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeMidiInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "UnsubscribeMidiInputFromField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn unsubscribe_midi_input_from_field_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "UnsubscribeMidiInputFromField",
                "graph_id": 1,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "UnsubscribeMidiInputFromField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad field index
        test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "UnsubscribeMidiInputFromField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 1,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad midi message
        test.call(serde_json::json!({
            "id": 5,
            "command": {
                "type": "SubscribeMidiInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 6,
            "command": {
                "type": "UnsubscribeMidiInputFromField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "message": {
                    "type": "BAD MIDI MESSAGE",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 6);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn subscribe_midi_output_to_pin() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeMidiOutputToPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "midi_output": "test-output",
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn subscribe_midi_output_to_pin_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeMidiOutputToPin",
                "graph_id": 1,
                "node_id": 0,
                "pin_index": 0,
                "midi_output": "test-output",
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad midi message
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "SubscribeMidiOutputToPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "midi_output": "test-output",
                "message": {
                    "type": "BAD MIDI MESSAGE",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_midi_output_subscriptions() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetMidiOutputSubscriptions",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "MidiOutputSubscriptions");
    }

    #[test]
    fn unsubscribe_midi_output_from_pin() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeMidiOutputToPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "midi_output": "test-output",
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "UnsubscribeMidiOutputFromPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "midi_output": "test-output",
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn unsubscribe_midi_output_from_pin_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "UnsubscribeMidiOutputFromPin",
                "graph_id": 1,
                "node_id": 0,
                "pin_index": 0,
                "midi_output": "test-output",
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "UnsubscribeMidiOutputFromPin",
                "graph_id": 0,
                "node_id": 1,
                "pin_index": 0,
                "midi_output": "test-output",
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad pin index
        let response = test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "UnsubscribeMidiOutputFromPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 1,
                "midi_output": "test-output",
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 3);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad midi output
        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "UnsubscribeMidiOutputFromPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "midi_output": "",
                "message": {
                    "type": "NoteOff",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad midi message
        let response = test.call(serde_json::json!({
            "id": 5,
            "command": {
                "type": "UnsubscribeMidiOutputFromPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "midi_output": "test-output",
                "message": {
                    "type": "BAD MIDI MESSAGE",
                    "note": 16,
                    "velocity": 16,
                },
            }
        }));

        assert_eq!(response["id"], 5);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_midi_input_name_from_id() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetMidiInputNameFromId",
                "id": "test-input",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_midi_input_name_from_id_bad() {
        let mut test = Test::new();

        // Bad id
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetMidiInputNameFromId",
                "id": "BAD INPUT ID",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_midi_output_name_from_id() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetMidiOutputNameFromId",
                "id": "test-output",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_midi_output_name_from_id_bad() {
        let mut test = Test::new();

        // Bad id
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetMidiOutputNameFromId",
                "id": "BAD OUTPUT ID",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn artnet_poll() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "ArtNetPoll",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn subscribe_artnet_input_to_field() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeArtnetInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn subscribe_artnet_input_to_field_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeArtnetInputToField",
                "graph_id": 1,
                "node_id": 0,
                "field_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "SubscribeArtnetInputToField",
                "graph_id": 0,
                "node_id": 1,
                "field_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad field index
        let response = test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "SubscribeArtnetInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 1,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 3);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad channel
        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "SubscribeArtnetInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "channel": 512,
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn unsubscribe_artnet_input_from_field() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeArtnetInputToField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "channel": 0,
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "UnsubscribeArtnetInputFromField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn unsubscribe_artnet_input_from_field_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "UnsubscribeArtnetInputFromField",
                "graph_id": 1,
                "node_id": 0,
                "field_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "UnsubscribeArtnetInputFromField",
                "graph_id": 0,
                "node_id": 1,
                "field_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad field index
        let response = test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "UnsubscribeArtnetInputFromField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 1,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 3);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad channel
        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "UnsubscribeArtnetInputFromField",
                "graph_id": 0,
                "node_id": 0,
                "field_index": 0,
                "channel": 512,
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_artnet_input_subscriptions() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetArtnetInputSubscriptions",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "ArtnetInputSubscriptions");
    }

    #[test]
    fn subscribe_artnet_output_to_pin() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeArtnetOutputToPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn subscribe_artnet_output_to_pin_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeArtnetOutputToPin",
                "graph_id": 1,
                "node_id": 0,
                "pin_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "SubscribeArtnetOutputToPin",
                "graph_id": 0,
                "node_id": 1,
                "pin_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad pin index
        let response = test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "SubscribeArtnetOutputToPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 1,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 3);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad channel
        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "SubscribeArtnetOutputToPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "channel": 512,
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn unsubscribe_artnet_output_from_pin() {
        let mut test = Test::new();

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "AddNode",
                "graph_id": 0,
                "kind": "ConstantNumber",
                "position": [0, 0],
            }
        }));

        test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "SubscribeArtnetOutputToPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "channel": 0,
            }
        }));

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "UnsubscribeArtnetOutputFromPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Acknowledge");
    }

    #[test]
    fn unsubscribe_artnet_output_from_pin_bad() {
        let mut test = Test::new();

        // Bad graph
        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "UnsubscribeArtnetOutputFromPin",
                "graph_id": 1,
                "node_id": 0,
                "pin_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad node
        let response = test.call(serde_json::json!({
            "id": 2,
            "command": {
                "type": "UnsubscribeArtnetOutputFromPin",
                "graph_id": 0,
                "node_id": 1,
                "pin_index": 0,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 2);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad pin index
        let response = test.call(serde_json::json!({
            "id": 3,
            "command": {
                "type": "UnsubscribeArtnetOutputFromPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 1,
                "channel": 0,
            }
        }));

        assert_eq!(response["id"], 3);
        assert_eq!(response["data"]["Ok"]["type"], "Error");

        // Bad channel
        let response = test.call(serde_json::json!({
            "id": 4,
            "command": {
                "type": "UnsubscribeArtnetOutputFromPin",
                "graph_id": 0,
                "node_id": 0,
                "pin_index": 0,
                "channel": 512,
            }
        }));

        assert_eq!(response["id"], 4);
        assert_eq!(response["data"]["Ok"]["type"], "Error");
    }

    #[test]
    fn get_artnet_output_subscriptions() {
        let mut test = Test::new();

        let response = test.call(serde_json::json!({
            "id": 1,
            "command": {
                "type": "GetArtnetOutputSubscriptions",
            }
        }));

        assert_eq!(response["id"], 1);
        assert_eq!(response["data"]["Ok"]["type"], "ArtnetOutputSubscriptions");
    }
}
