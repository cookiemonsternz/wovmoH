use std::{
    collections::HashMap,
    sync::{Arc, RwLock, mpsc::Sender},
};

use midir::{Ignore, MidiInput, MidiInputConnection, MidiOutput, MidiOutputConnection};
use serde::{Deserialize, Serialize};

use crate::{
    core::{graph::GraphId, graph_manager::GraphManager, node::NodeId},
    types::data_type::DataValue,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MidiDataMessage {
    NoteOff {
        note: Option<u8>,
        velocity: Option<u8>,
    },
    NoteOn {
        note: Option<u8>,
        velocity: Option<u8>,
    },
    PolyphonicAftertouch {
        note: Option<u8>,
        pressure: Option<u8>,
    },
    ControlChange {
        controller: Option<u8>,
        data: Option<u8>,
    },
    ProgramChange {
        program: Option<u8>,
    },
    ChannelAftertouch {
        pressure: Option<u8>,
    },
    PitchWheel {
        pitch: Option<u16>,
    },
    SysEx,
}

impl MidiDataMessage {
    pub fn to_raw(&self) -> [u8; 3] {
        match self {
            MidiDataMessage::NoteOff { note, velocity } => {
                [0b10000000, note.unwrap(), velocity.unwrap()]
            }
            MidiDataMessage::NoteOn { note, velocity } => {
                [0b10010000, note.unwrap(), velocity.unwrap()]
            }
            MidiDataMessage::PolyphonicAftertouch { note, pressure } => {
                [0b10100000, note.unwrap(), pressure.unwrap()]
            }
            MidiDataMessage::ControlChange { controller, data } => {
                [0b10110000, controller.unwrap(), data.unwrap()]
            }
            MidiDataMessage::ProgramChange { program: _ } => todo!(),
            MidiDataMessage::ChannelAftertouch { pressure: _ } => todo!(),
            MidiDataMessage::PitchWheel { pitch: _ } => todo!(),
            MidiDataMessage::SysEx => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortDescriptor {
    name: String,
    id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MidiInputFieldSubscriptionDTO {
    pub graph_id: GraphId,
    pub node_id: NodeId,
    pub field_index: usize,
    pub message: MidiDataMessage,
}

#[derive(Debug, PartialEq)]
pub struct MidiInputFieldSubscription {
    pub graph: GraphId,
    pub field: usize,
    pub message: MidiDataMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MidiOutputPinSubscriptionDTO {
    pub graph_id: GraphId,
    pub node_id: NodeId,
    pub pin_index: usize,
    pub midi_output: String,
    pub message: MidiDataMessage,
}

#[derive(Debug, PartialEq)]
pub struct MidiOutputPinSubscription {
    pub midi_output: String,
    pub graph: GraphId,
    pub pin: usize,
    pub message: MidiDataMessage,
}

pub struct MidiEvent {
    pub graph: GraphId,
    pub field: usize,
    pub message: MidiDataMessage,
}

pub struct MidiManager {
    input_connections: HashMap<String, MidiInputConnection<()>>,
    output_connections: HashMap<String, MidiOutputConnection>,
    pub input_subscriptions: Arc<RwLock<Vec<MidiInputFieldSubscription>>>,
    pub output_subscriptions: Arc<RwLock<Vec<MidiOutputPinSubscription>>>,
    tx: Sender<MidiEvent>,
}

fn matches_filter<T: PartialEq + Clone>(filter: &Option<T>, actual: &Option<T>) -> bool {
    filter
        .clone()
        .is_none_or(|expected| expected == actual.clone().unwrap())
}

impl MidiManager {
    pub fn new(tx: Sender<MidiEvent>) -> MidiManager {
        MidiManager {
            input_connections: HashMap::new(),
            output_connections: HashMap::new(),
            input_subscriptions: Arc::new(RwLock::new(Vec::new())),
            output_subscriptions: Arc::new(RwLock::new(Vec::new())),
            tx,
        }
    }

    pub fn subscribe_input(&mut self, subscription: MidiInputFieldSubscription) {
        self.input_subscriptions.write().unwrap().push(subscription);
    }

    pub fn unsubscribe_input(&mut self, subscription: MidiInputFieldSubscription) {
        let index = self
            .input_subscriptions
            .read()
            .unwrap()
            .iter()
            .position(|x| *x == subscription)
            .unwrap();
        self.input_subscriptions.write().unwrap().swap_remove(index);
    }

    pub fn subscribe_output(&mut self, subscription: MidiOutputPinSubscription) {
        self.output_subscriptions
            .write()
            .unwrap()
            .push(subscription);
    }

    pub fn unsubscribe_output(&mut self, subscription: MidiOutputPinSubscription) {
        let index = self
            .output_subscriptions
            .read()
            .unwrap()
            .iter()
            .position(|x| *x == subscription)
            .unwrap();
        self.output_subscriptions
            .write()
            .unwrap()
            .swap_remove(index);
    }

    fn parse_midi_message(message: &[u8]) -> Result<MidiDataMessage, ()> {
        // println!("{:b}", message[0] & 0b11110000);
        let midi_message: Result<MidiDataMessage, ()> = match (message[0] & 0b11110000) >> 4 {
            0b1000 => Ok(MidiDataMessage::NoteOff {
                note: Some(message[1]),
                velocity: Some(message[2]),
            }),
            0b1001 => Ok(MidiDataMessage::NoteOn {
                note: Some(message[1]),
                velocity: Some(message[2]),
            }),
            0b1010 => Ok(MidiDataMessage::PolyphonicAftertouch {
                note: Some(message[1]),
                pressure: Some(message[2]),
            }),
            0b1011 => Ok(MidiDataMessage::ControlChange {
                controller: Some(message[1]),
                data: Some(message[2]),
            }),
            0b1100 => Ok(MidiDataMessage::ProgramChange {
                program: Some(message[1]),
            }),
            0b1101 => Ok(MidiDataMessage::ChannelAftertouch {
                pressure: Some(message[1]),
            }),
            0b1110 => Ok(MidiDataMessage::PitchWheel {
                pitch: Some(((message[2] as u16) << 7) | (message[1] as u16)),
            }),
            0b1111 => Ok(MidiDataMessage::SysEx),
            _ => Err(()),
        };

        if midi_message.is_err() {
            eprintln!("Unrecognised midi message {:?}", message);
            return Err(());
        }
        Ok(midi_message.unwrap())
    }

    pub fn handle_midi_message(
        subscriptions: &[MidiInputFieldSubscription],
        message: MidiDataMessage,
    ) -> Vec<MidiEvent> {
        let mut events = Vec::new();
        for subscription in subscriptions {
            let matches = match (&subscription.message, &message) {
                (
                    MidiDataMessage::NoteOff {
                        note: filter_note,
                        velocity: filter_velocity,
                    },
                    MidiDataMessage::NoteOff { note, velocity },
                ) => matches_filter(filter_note, note) && matches_filter(filter_velocity, velocity),
                (
                    MidiDataMessage::NoteOn {
                        note: filter_note,
                        velocity: filter_velocity,
                    },
                    MidiDataMessage::NoteOn { note, velocity },
                ) => matches_filter(filter_note, note) && matches_filter(filter_velocity, velocity),
                (
                    MidiDataMessage::PolyphonicAftertouch {
                        note: filter_note,
                        pressure: filter_pressure,
                    },
                    MidiDataMessage::PolyphonicAftertouch { note, pressure },
                ) => matches_filter(filter_note, note) && matches_filter(filter_pressure, pressure),
                (
                    MidiDataMessage::ControlChange {
                        controller: filter_controller,
                        data: filter_data,
                    },
                    MidiDataMessage::ControlChange { controller, data },
                ) => {
                    matches_filter(filter_controller, controller)
                        && matches_filter(filter_data, data)
                }
                (
                    MidiDataMessage::ProgramChange {
                        program: filter_program,
                    },
                    MidiDataMessage::ProgramChange { program },
                ) => matches_filter(filter_program, program),
                (
                    MidiDataMessage::ChannelAftertouch {
                        pressure: filter_pressure,
                    },
                    MidiDataMessage::ChannelAftertouch { pressure },
                ) => matches_filter(filter_pressure, pressure),
                (
                    MidiDataMessage::PitchWheel {
                        pitch: filter_pitch,
                    },
                    MidiDataMessage::PitchWheel { pitch },
                ) => matches_filter(filter_pitch, pitch),
                (MidiDataMessage::SysEx, MidiDataMessage::SysEx) => true,
                _ => false,
            };

            if !matches {
                continue;
            };

            events.push(MidiEvent {
                graph: subscription.graph,
                field: subscription.field,
                message: message.clone(),
            });
        }

        events
    }

    pub fn print_available_input_ports() {
        let mut midi_input = MidiInput::new("Test input").expect("Could not create MIDI input");
        midi_input.ignore(Ignore::None);

        println!("---- Available MIDI Input Ports ----");
        for (i, p) in midi_input.ports().iter().enumerate() {
            println!(
                "{}: {} (ID: \"{}\")",
                i,
                midi_input
                    .port_name(p)
                    .unwrap_or("Could not get port name".to_string()),
                p.id()
            );
        }
    }

    pub fn print_available_output_ports() {
        let midi_output =
            MidiOutput::new("midir test output").expect("Could not create MIDI output");

        println!("---- Available MIDI Output Ports ----");
        for (i, p) in midi_output.ports().iter().enumerate() {
            println!(
                "{}: {} (ID: \"{}\")",
                i,
                midi_output
                    .port_name(p)
                    .unwrap_or("Could not get port name".to_string()),
                p.id()
            );
        }
    }

    pub fn get_available_input_ports() -> Vec<PortDescriptor> {
        let mut midi_input = MidiInput::new("Test input").expect("Could not create MIDI input");
        midi_input.ignore(Ignore::None);

        midi_input
            .ports()
            .iter()
            .map(|x| PortDescriptor {
                name: midi_input.port_name(x).expect("Could not get port name"),
                id: x.id(),
            })
            .collect()
    }

    pub fn get_available_output_ports() -> Vec<PortDescriptor> {
        let midi_output =
            MidiOutput::new("midir test output").expect("Could not create MIDI output");

        midi_output
            .ports()
            .iter()
            .map(|x| PortDescriptor {
                name: midi_output.port_name(x).expect("Could not get port name"),
                id: x.id(),
            })
            .collect()
    }

    pub fn connect_input(&mut self, port_id: String) -> Result<(), String> {
        let mut midi_input = match MidiInput::new("Test input") {
            Ok(input) => input,
            Err(_) => return Err("Error initializing midi input.".to_string()),
        };
        midi_input.ignore(Ignore::None);

        let _port = &midi_input.find_port_by_id(&port_id);
        let in_port = match _port {
            Some(port) => port,
            None => return Err(format!("No midi input port availible with id {}", port_id)),
        };

        let in_port_name = match midi_input.port_name(in_port) {
            Ok(name) => name,
            Err(_) => return Err(format!("Error getting port name for id {}", port_id)),
        };

        println!("Connecting to: {}", in_port_name);

        let subscriptions = Arc::clone(&self.input_subscriptions);
        let tx = self.tx.clone();

        // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
        self.input_connections.insert(
            in_port.id(),
            midi_input
                .connect(
                    in_port,
                    "midir-read-input",
                    move |_stamp, message, _| {
                        // println!("{}: {:?} (len = {})", stamp, message, message.len());
                        match MidiManager::parse_midi_message(message) {
                            Ok(message) => {
                                let events: Vec<MidiEvent> = {
                                    let subscriptions = subscriptions.read().unwrap();
                                    MidiManager::handle_midi_message(&subscriptions, message)
                                };
                                for event in events {
                                    tx.send(event).unwrap();
                                }
                            }
                            Err(_) => return,
                        };
                    },
                    (),
                )
                .map_err(|_| format!("Could not connect to midi input: {}", in_port_name))?,
        );

        Ok(())
    }

    pub fn connect_output(&mut self, port_id: String) -> Result<(), String> {
        let midi_output = MidiOutput::new("Test output")
            .map_err(|_| "Error initializing midi output".to_string())?;

        let _port = &midi_output.find_port_by_id(&port_id);
        let out_port = match _port {
            Some(port) => port,
            None => {
                return Err(format!(
                    "No midi output port availabile with id {}",
                    port_id
                ));
            }
        };

        let out_port_name = match midi_output.port_name(out_port) {
            Ok(name) => name,
            Err(_) => return Err(format!("Error getting port name for id {}", port_id)),
        };

        println!("Connecting to: {}", out_port_name);

        self.output_connections.insert(
            out_port.id(),
            midi_output
                .connect(out_port, "midir-write-output")
                .map_err(|_| format!("Error connecting to midi output port with id {}", port_id))?,
        );

        Ok(())
    }

    pub fn send_output_events(&mut self, graphs: &GraphManager) -> Result<(), String> {
        // let mut lock = app.lock();
        // let app = lock.as_mut().unwrap();

        for output_subscription in self.output_subscriptions.read().unwrap().iter() {
            let graph = graphs.try_get_graph(output_subscription.graph)?;
            let pin = graph.get_output(output_subscription.pin);
            let value = pin.value.clone();

            let midi_message = match output_subscription.message {
                MidiDataMessage::NoteOff { note, velocity: _ } => {
                    let vel = match value {
                        DataValue::Number(x) => x as u8,
                        DataValue::Boolean(x) => {
                            if x {
                                255
                            } else {
                                0
                            }
                        }
                        DataValue::Color(_color) => todo!(),
                        DataValue::Vector3(_vec3) => todo!(),
                        DataValue::Point3(_vec3) => todo!(),
                    };
                    MidiDataMessage::NoteOff {
                        note,
                        velocity: Some(vel),
                    }
                }
                MidiDataMessage::NoteOn { note, velocity: _ } => {
                    let vel = match value {
                        DataValue::Number(x) => x as u8,
                        DataValue::Boolean(x) => {
                            if x {
                                255
                            } else {
                                0
                            }
                        }
                        DataValue::Color(_color) => todo!(),
                        DataValue::Vector3(_vec3) => todo!(),
                        DataValue::Point3(_vec3) => todo!(),
                    };
                    MidiDataMessage::NoteOn {
                        note,
                        velocity: Some(vel),
                    }
                }
                MidiDataMessage::PolyphonicAftertouch {
                    note: _,
                    pressure: _,
                } => todo!(),
                MidiDataMessage::ControlChange {
                    controller,
                    data: _,
                } => {
                    let dat = match value {
                        DataValue::Number(x) => x as u8,
                        DataValue::Boolean(x) => {
                            if x {
                                255
                            } else {
                                0
                            }
                        }
                        DataValue::Color(_color) => todo!(),
                        DataValue::Vector3(_vec3) => todo!(),
                        DataValue::Point3(_vec3) => todo!(),
                    };
                    MidiDataMessage::ControlChange {
                        controller,
                        data: Some(dat),
                    }
                }
                MidiDataMessage::ProgramChange { program: _ } => todo!(),
                MidiDataMessage::ChannelAftertouch { pressure: _ } => todo!(),
                MidiDataMessage::PitchWheel { pitch: _ } => todo!(),
                MidiDataMessage::SysEx => todo!(),
            };

            println!("Sending Midi Message {:?}", &midi_message);

            self.output_connections
                .get_mut(&output_subscription.midi_output)
                .unwrap()
                .send(&midi_message.to_raw())
                .expect("Could not send midi message");
        }

        Ok(())
    }

    pub fn disconnect_input(&mut self, port_id: String) -> Result<(), String> {
        match self.input_connections.remove(&port_id) {
            Some(connection) => connection.close(),
            None => return Err(format!("No connection to port id {}", port_id)),
        };
        Ok(())
    }

    pub fn disconnect_output(&mut self, port_id: String) -> Result<(), String> {
        match self.output_connections.remove(&port_id) {
            Some(connection) => connection.close(),
            None => return Err(format!("No connection to port id {}", port_id)),
        };
        Ok(())
    }

    pub fn get_connected_inputs(&self) -> Result<Vec<PortDescriptor>, String> {
        let mut midi_input = match MidiInput::new("Test input") {
            Ok(input) => input,
            Err(_) => return Err("Error initializing midi input.".to_string()),
        };
        midi_input.ignore(Ignore::None);

        let mut connections = Vec::new();

        for connection in self.input_connections.iter() {
            connections.push(PortDescriptor {
                name: match midi_input.port_name(match &midi_input.find_port_by_id(connection.0) {
                    Some(port) => port,
                    None => {
                        return Err(format!(
                            "No midi input port availabile with id {}",
                            connection.0
                        ));
                    }
                }) {
                    Ok(name) => name,
                    Err(_) => {
                        return Err(format!(
                            "Could not find port name for port id {}",
                            connection.0
                        ));
                    }
                },
                id: connection.0.to_string(),
            });
        }

        Ok(connections)
    }

    pub fn get_connected_outputs(&self) -> Result<Vec<PortDescriptor>, String> {
        let midi_output = MidiOutput::new("Test output")
            .map_err(|_| "Could not create midi output port".to_string())?;

        let mut connections = Vec::new();

        for connection in self.output_connections.iter() {
            connections.push(PortDescriptor {
                name: match midi_output.port_name(
                    match &midi_output.find_port_by_id(connection.0) {
                        Some(port) => port,
                        None => {
                            return Err(format!(
                                "No midi output port availabile with id {}",
                                connection.0
                            ));
                        }
                    },
                ) {
                    Ok(name) => name,
                    Err(_) => {
                        return Err(format!(
                            "Could not find midi output port name for id {}",
                            connection.0
                        ));
                    }
                },
                id: connection.0.to_string(),
            });
        }

        Ok(connections)
    }

    pub fn get_input_name_from_id(id: String) -> Result<String, String> {
        let mut midi_input =
            MidiInput::new("Test input").map_err(|_| "Could not create midi input".to_string())?;
        midi_input.ignore(Ignore::None);

        Ok(midi_input
            .port_name(match &midi_input.find_port_by_id(&id) {
                Some(port) => port,
                None => return Err(format!("No midi input port found with id {}", id)),
            })
            .map_err(|_| format!("Could not find midi input port name with id {}", id))?)
    }

    pub fn get_output_name_from_id(id: String) -> Result<String, String> {
        let midi_output = MidiOutput::new("Test output")
            .map_err(|_| "Could not create MIDI output".to_string())?;

        Ok(midi_output
            .port_name(match &midi_output.find_port_by_id(&id) {
                Some(port) => port,
                None => return Err(format!("No midi output port found with id {}", id)),
            })
            .map_err(|_| format!("Could not get name of port with id {}", id))?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_midi_manager_init() {
        use crate::io::midi::{MidiEvent, MidiManager};
        use std::sync::mpsc;

        let (tx, _rx) = mpsc::channel::<MidiEvent>();
        let _manager = MidiManager::new(tx);
    }

    #[test]
    fn test_midi_manager_get_inputs_outputs() {
        use crate::io::midi::{MidiEvent, MidiManager};
        use std::sync::mpsc;

        let (tx, _rx) = mpsc::channel::<MidiEvent>();
        let _manager = MidiManager::new(tx);

        let _inputs = MidiManager::get_available_input_ports();
        let _outputs = MidiManager::get_available_output_ports();
    }

    #[test]
    fn test_midi_manager_connect_disconnect_bad_input() {
        use crate::io::midi::{MidiEvent, MidiManager};
        use std::sync::mpsc;

        let (tx, _rx) = mpsc::channel::<MidiEvent>();
        let mut manager = MidiManager::new(tx);

        assert!(
            manager
                .connect_input("BAD_MIDI_INPUT_ID".to_string())
                .is_err()
        );

        assert!(
            manager
                .connect_output("BAD_MIDI_OUTPUT_ID".to_string())
                .is_err()
        );

        assert!(
            manager
                .disconnect_input("BAD_MIDI_INPUT_ID".to_string())
                .is_err()
        );

        assert!(
            manager
                .disconnect_output("BAD_MIDI_OUTPUT_ID".to_string())
                .is_err()
        );
    }
}
