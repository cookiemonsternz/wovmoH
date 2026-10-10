use std::sync::mpsc::{self, Receiver};

use crate::{
    core::graph_manager::GraphManager,
    io::midi::{MIDIDataMessage, MidiEvent, MidiManager},
};

pub struct App {
    pub graphs: GraphManager,
    pub midi: MidiManager,

    midi_rx: Receiver<MidiEvent>,
}

impl App {
    pub fn new() -> App {
        let (midi_tx, midi_rx) = mpsc::channel::<MidiEvent>();

        let app = App {
            graphs: GraphManager::new(),
            midi: MidiManager::new(midi_tx),
            midi_rx,
        };

        MidiManager::print_available_input_ports();
        MidiManager::print_available_output_ports();

        app
    }

    pub fn process_midi_events(&mut self) {
        while let Ok(event) = self.midi_rx.try_recv() {
            self.graphs.handle_midi_event(event);
        }
    }
}
