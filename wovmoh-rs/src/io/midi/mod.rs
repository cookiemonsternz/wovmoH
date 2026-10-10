use std::{
    any::Any,
    collections::HashMap,
    ops::Index,
    sync::{Arc, RwLock, mpsc::Sender},
};

use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort, MidiOutput};
use serde::{Deserialize, Serialize};

use crate::core::{graph::GraphId, node::NodeId};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum MIDIDataMessage {
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
    pub message: MIDIDataMessage,
}

#[derive(Debug, PartialEq)]
pub struct MidiInputFieldSubscription {
    pub graph: GraphId,
    pub field: usize,
    pub message: MIDIDataMessage,
}

pub struct MidiEvent {
    pub graph: GraphId,
    pub field: usize,
    pub message: MIDIDataMessage,
}

pub struct MidiManager {
    input_connections: HashMap<String, MidiInputConnection<()>>,
    pub input_subscriptions: Arc<RwLock<Vec<MidiInputFieldSubscription>>>,
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
            input_subscriptions: Arc::new(RwLock::new(Vec::new())),
            tx,
        }
    }

    pub fn subscribe(&mut self, subscription: MidiInputFieldSubscription) {
        self.input_subscriptions.write().unwrap().push(subscription);
    }

    pub fn unsubscribe(&mut self, subscription: MidiInputFieldSubscription) {
        let index = self
            .input_subscriptions
            .read()
            .unwrap()
            .iter()
            .position(|x| *x == subscription)
            .unwrap();
        self.input_subscriptions.write().unwrap().swap_remove(index);
    }

    fn parse_midi_message(message: &[u8]) -> MIDIDataMessage {
        // println!("{:b}", message[0] & 0b11110000);
        let midi_message: Result<MIDIDataMessage, ()> = match (message[0] & 0b11110000) >> 4 {
            0b1000 => Ok(MIDIDataMessage::NoteOff {
                note: Some(message[1]),
                velocity: Some(message[2]),
            }),
            0b1001 => Ok(MIDIDataMessage::NoteOn {
                note: Some(message[1]),
                velocity: Some(message[2]),
            }),
            0b1010 => Ok(MIDIDataMessage::PolyphonicAftertouch {
                note: Some(message[1]),
                pressure: Some(message[2]),
            }),
            0b1011 => Ok(MIDIDataMessage::ControlChange {
                controller: Some(message[1]),
                data: Some(message[2]),
            }),
            0b1100 => Ok(MIDIDataMessage::ProgramChange {
                program: Some(message[1]),
            }),
            0b1101 => Ok(MIDIDataMessage::ChannelAftertouch {
                pressure: Some(message[1]),
            }),
            0b1110 => Ok(MIDIDataMessage::PitchWheel {
                pitch: Some(((message[2] as u16) << 7) | (message[1] as u16)),
            }),
            0b1111 => Ok(MIDIDataMessage::SysEx),
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
        message: MIDIDataMessage,
    ) -> Vec<MidiEvent> {
        let mut events = Vec::new();
        for subscription in subscriptions {
            let matches = match (&subscription.message, &message) {
                (
                    MIDIDataMessage::NoteOff {
                        note: filter_note,
                        velocity: filter_velocity,
                    },
                    MIDIDataMessage::NoteOff { note, velocity },
                ) => matches_filter(filter_note, note) && matches_filter(filter_velocity, velocity),
                (
                    MIDIDataMessage::NoteOn {
                        note: filter_note,
                        velocity: filter_velocity,
                    },
                    MIDIDataMessage::NoteOn { note, velocity },
                ) => matches_filter(filter_note, note) && matches_filter(filter_velocity, velocity),
                (
                    MIDIDataMessage::PolyphonicAftertouch {
                        note: filter_note,
                        pressure: filter_pressure,
                    },
                    MIDIDataMessage::PolyphonicAftertouch { note, pressure },
                ) => matches_filter(filter_note, note) && matches_filter(filter_pressure, pressure),
                (
                    MIDIDataMessage::ControlChange {
                        controller: filter_controller,
                        data: filter_data,
                    },
                    MIDIDataMessage::ControlChange { controller, data },
                ) => {
                    matches_filter(filter_controller, controller)
                        && matches_filter(filter_data, data)
                }
                (
                    MIDIDataMessage::ProgramChange {
                        program: filter_program,
                    },
                    MIDIDataMessage::ProgramChange { program },
                ) => matches_filter(filter_program, program),
                (
                    MIDIDataMessage::ChannelAftertouch {
                        pressure: filter_pressure,
                    },
                    MIDIDataMessage::ChannelAftertouch { pressure },
                ) => matches_filter(filter_pressure, pressure),
                (
                    MIDIDataMessage::PitchWheel {
                        pitch: filter_pitch,
                    },
                    MIDIDataMessage::PitchWheel { pitch },
                ) => matches_filter(filter_pitch, pitch),
                (MIDIDataMessage::SysEx, MIDIDataMessage::SysEx) => true,
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
                        let message: MIDIDataMessage = MidiManager::parse_midi_message(message);
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
}
