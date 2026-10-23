use std::{
    any::Any,
    collections::HashMap,
    ops::Index,
    sync::{Arc, Mutex, RwLock, mpsc::Sender},
};

use midir::{
    Ignore, MidiInput, MidiInputConnection, MidiInputPort, MidiOutput, MidiOutputConnection,
};
use serde::{Deserialize, Serialize};

use crate::{
    app::App,
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
            MidiDataMessage::ProgramChange { program } => todo!(),
            MidiDataMessage::ChannelAftertouch { pressure } => todo!(),
            MidiDataMessage::PitchWheel { pitch } => todo!(),
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

    fn parse_midi_message(message: &[u8]) -> MidiDataMessage {
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

        // message.unwrap_or_else(|x| {
        //     println!("Unrecognised midi message {:?}", message);
        //     panic!();
        // })
        if midi_message.is_err() {
            println!("Unrecognised midi message {:?}", message);
            panic!();
        }
        midi_message.unwrap()
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

    pub fn connect_input(&mut self, port_id: String) {
        let mut midi_input = MidiInput::new("Test input").expect("Could not create MIDI input");
        midi_input.ignore(Ignore::None);

        let in_port = &midi_input
            .find_port_by_id(&port_id)
            .expect("No available port with id");

        let in_port_name = midi_input
            .port_name(in_port)
            .expect("Could not get port name");

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
                    move |stamp, message, _| {
                        // println!("{}: {:?} (len = {})", stamp, message, message.len());
                        let message: MidiDataMessage = MidiManager::parse_midi_message(message);
                        let events: Vec<MidiEvent> = {
                            let subscriptions = subscriptions.read().unwrap();
                            MidiManager::handle_midi_message(&subscriptions, message)
                        };
                        for event in events {
                            tx.send(event);
                        }
                    },
                    (),
                )
                .expect("Could not connect to port."),
        );
    }

    pub fn connect_output(&mut self, port_id: String) {
        let mut midi_output = MidiOutput::new("Test output").expect("Could not create MIDI output");

        let out_port = &midi_output
            .find_port_by_id(&port_id)
            .expect("No available port with id");

        let out_port_name = midi_output
            .port_name(out_port)
            .expect("Could not get port name");

        println!("Connecting to: {}", out_port_name);

        self.output_connections.insert(
            out_port.id(),
            midi_output
                .connect(out_port, "midir-write-output")
                .expect("Could not connect to port."),
        );
    }

    pub fn send_output_events(&mut self, graphs: &GraphManager) {
        // let mut lock = app.lock();
        // let app = lock.as_mut().unwrap();

        for output_subscription in self.output_subscriptions.read().unwrap().iter() {
            let graph = graphs.get_graph(output_subscription.graph);
            let pin = graph.get_output(output_subscription.pin);
            let value = pin.value.clone();

            let midi_message = match output_subscription.message {
                MidiDataMessage::NoteOff { note, velocity } => {
                    let vel = match value {
                        DataValue::Number(x) => x as u8,
                        DataValue::Boolean(x) => {
                            if x {
                                255
                            } else {
                                0
                            }
                        }
                        DataValue::Color(color) => todo!(),
                        DataValue::Vector3(vec3) => todo!(),
                        DataValue::Point3(vec3) => todo!(),
                    };
                    MidiDataMessage::NoteOff {
                        note,
                        velocity: Some(vel),
                    }
                }
                MidiDataMessage::NoteOn { note, velocity } => {
                    let vel = match value {
                        DataValue::Number(x) => x as u8,
                        DataValue::Boolean(x) => {
                            if x {
                                255
                            } else {
                                0
                            }
                        }
                        DataValue::Color(color) => todo!(),
                        DataValue::Vector3(vec3) => todo!(),
                        DataValue::Point3(vec3) => todo!(),
                    };
                    MidiDataMessage::NoteOn {
                        note,
                        velocity: Some(vel),
                    }
                }
                MidiDataMessage::PolyphonicAftertouch { note, pressure } => todo!(),
                MidiDataMessage::ControlChange { controller, data } => {
                    let dat = match value {
                        DataValue::Number(x) => x as u8,
                        DataValue::Boolean(x) => {
                            if x {
                                255
                            } else {
                                0
                            }
                        }
                        DataValue::Color(color) => todo!(),
                        DataValue::Vector3(vec3) => todo!(),
                        DataValue::Point3(vec3) => todo!(),
                    };
                    MidiDataMessage::ControlChange {
                        controller,
                        data: Some(dat),
                    }
                }
                MidiDataMessage::ProgramChange { program } => todo!(),
                MidiDataMessage::ChannelAftertouch { pressure } => todo!(),
                MidiDataMessage::PitchWheel { pitch } => todo!(),
                MidiDataMessage::SysEx => todo!(),
            };

            println!("Sending Midi Message {:?}", &midi_message);

            self.output_connections
                .get_mut(&output_subscription.midi_output)
                .unwrap()
                .send(&midi_message.to_raw())
                .expect("Could not send midi message");
        }
    }

    pub fn disconnect_input(&mut self, port_id: String) {
        self.input_connections.remove(&port_id).unwrap().close();
    }

    pub fn get_connected_inputs(&self) -> Vec<PortDescriptor> {
        let mut midi_input = MidiInput::new("Test input").expect("Could not create MIDI input");
        midi_input.ignore(Ignore::None);

        let mut connections = Vec::new();

        for connection in self.input_connections.iter() {
            connections.push(PortDescriptor {
                name: midi_input
                    .port_name(
                        &midi_input
                            .find_port_by_id(connection.0)
                            .expect("Could not find port from id"),
                    )
                    .expect("Could not get port name"),
                id: connection.0.to_string(),
            });
        }

        connections
    }

    pub fn get_connected_outputs(&self) -> Vec<PortDescriptor> {
        let midi_output = MidiOutput::new("Test output").expect("Could not create MIDI output");

        let mut connections = Vec::new();

        for connection in self.output_connections.iter() {
            connections.push(PortDescriptor {
                name: midi_output
                    .port_name(
                        &midi_output
                            .find_port_by_id(connection.0)
                            .expect("Could not find port from id"),
                    )
                    .expect("Could not get port name"),
                id: connection.0.to_string(),
            });
        }

        connections
    }

    pub fn get_input_name_from_id(id: String) -> String {
        let mut midi_input = MidiInput::new("Test input").expect("Could not create MIDI input");
        midi_input.ignore(Ignore::None);

        midi_input
            .port_name(
                &midi_input
                    .find_port_by_id(&id)
                    .expect("Could not find port with id"),
            )
            .expect("Could not get name of port")
    }

    pub fn get_output_name_from_id(id: String) -> String {
        let midi_output = MidiOutput::new("Test output").expect("Could not create MIDI output");

        midi_output
            .port_name(
                &midi_output
                    .find_port_by_id(&id)
                    .expect("Could not find port with id"),
            )
            .expect("Could not get name of port")
    }
}
