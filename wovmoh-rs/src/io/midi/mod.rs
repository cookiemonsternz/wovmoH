use std::any::Any;

use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort, MidiOutput};

use crate::core::graph::GraphId;

#[derive(Debug, Clone)]
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
    input_connections: Vec<MidiInputConnection<()>>,
    input_subscriptions: Vec<MidiInputFieldSubscription>,
}

fn matches_filter<T: PartialEq + Clone>(filter: &Option<T>, actual: &Option<T>) -> bool {
    filter
        .clone()
        .is_none_or(|expected| expected == actual.clone().unwrap())
}

impl MidiManager {
    pub fn new() -> MidiManager {
        MidiManager {
            input_connections: Vec::new(),
            input_subscriptions: Vec::new(),
        }
    }

    pub fn subscribe(&mut self, subscription: MidiInputFieldSubscription) {
        self.input_subscriptions.push(subscription);
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

    pub fn handle_midi_message(&self, message: MIDIDataMessage) -> Vec<MidiEvent> {
        let mut events = Vec::new();
        for subscription in &self.input_subscriptions {
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
                    matches_filter(filter_controller, filter_data)
                        && matches_filter(controller, data)
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

    pub fn connect_input(&mut self, port_index: usize) {
        let mut midi_input = MidiInput::new("Test input").expect("Could not create MIDI input");
        midi_input.ignore(Ignore::None);

        let in_port = &midi_input.ports()[port_index];

        let in_port_name = midi_input
            .port_name(in_port)
            .expect("Could not get port name");

        println!("Connecting to: {}", in_port_name);

        // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
        self.input_connections.push(
            midi_input
                .connect(
                    in_port,
                    "midir-read-input",
                    move |stamp, message, _| {
                        // println!("{}: {:?} (len = {})", stamp, message, message.len());
                        let message: MIDIDataMessage = MidiManager::parse_midi_message(message);
                        println!("{:?}", message)
                    },
                    (),
                )
                .expect("Could not connect to port."),
        );
    }
}
