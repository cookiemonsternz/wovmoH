use std::sync::{
    Arc, Mutex,
    mpsc::{self, Receiver},
};

use crate::{
    core::graph_manager::GraphManager,
    io::{
        artnet::{ArtnetManager, WovmohArtCommand},
        midi::{MidiDataMessage, MidiEvent, MidiManager},
    },
};

pub struct App {
    pub graphs: GraphManager,
    pub midi: MidiManager,
    pub artnet: ArtnetManager,

    midi_rx: Receiver<MidiEvent>,
    artnet_rx: Receiver<WovmohArtCommand>,
}

impl App {
    pub fn new() -> App {
        let (midi_tx, midi_rx) = mpsc::channel::<MidiEvent>();
        let (artnet_tx, artnet_rx) = mpsc::channel::<WovmohArtCommand>();

        let app = App {
            graphs: GraphManager::new(),
            midi: MidiManager::new(midi_tx),
            artnet: ArtnetManager::new(artnet_tx),
            midi_rx,
            artnet_rx,
        };

        MidiManager::print_available_input_ports();
        MidiManager::print_available_output_ports();

        app.artnet.poll();

        app
    }

    pub fn process_midi_events(&mut self) {
        while let Ok(event) = self.midi_rx.try_recv() {
            self.graphs.handle_midi_event(event);
        }
    }

    pub fn process_artnet_events(&mut self) {
        while let Ok(event) = self.artnet_rx.try_recv() {
            self.graphs.handle_artnet_event(event);
        }
    }

    pub fn event_loop(&mut self) {
        self.process_midi_events();
        self.process_artnet_events();

        for graph in self.graphs.iter_mut() {
            graph.process();
        }

        self.midi.send_output_events(&self.graphs);
    }
}
