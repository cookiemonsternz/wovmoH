use std::any::Any;

use midir::{Ignore, MidiInput, MidiInputConnection, MidiInputPort, MidiOutput};

#[derive(Debug)]
pub enum MIDIDataMessage {
    NoteOff { note: u8, velocity: u8 },
    NoteOn { note: u8, velocity: u8 },
    PolyphonicAftertouch { note: u8, pressure: u8 },
    ControlChange { controller: u8, data: u8 },
    ProgramChange { program: u8 },
    ChannelAftertouch { pressure: u8 },
    PitchWheel { pitch: u16 },
    SysEx,
}

pub struct MidiInputFieldSubscription {
    pub field: usize,
    pub message: MIDIDataMessage,
}

pub struct MidiEvent {
    pub field: usize,
    pub message: MIDIDataMessage,
}

pub struct MidiManager {
    input_connections: Vec<MidiInputConnection<()>>,
    input_subscriptions: Vec<MidiInputFieldSubscription>,
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
                note: message[1],
                velocity: message[2],
            }),
            0b1001 => Ok(MIDIDataMessage::NoteOn {
                note: message[1],
                velocity: message[2],
            }),
            0b1010 => Ok(MIDIDataMessage::PolyphonicAftertouch {
                note: message[1],
                pressure: message[2],
            }),
            0b1011 => Ok(MIDIDataMessage::ControlChange {
                controller: message[1],
                data: message[2],
            }),
            0b1100 => Ok(MIDIDataMessage::ProgramChange {
                program: message[1],
            }),
            0b1101 => Ok(MIDIDataMessage::ChannelAftertouch {
                pressure: message[1],
            }),
            0b1110 => Ok(MIDIDataMessage::PitchWheel {
                pitch: ((message[2] as u16) << 7) | (message[1] as u16),
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

    fn handle_midi_message(&self, message: MIDIDataMessage) {
        for subscription in &self.input_subscriptions {
            if std::mem::discriminant(&subscription.message) == std::mem::discriminant(&message) {}
        }
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
