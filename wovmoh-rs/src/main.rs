use std::sync::{Arc, Mutex};

use crate::{app::App, io::midi::input::MidiManager, transport::tcp};

pub mod api;
pub mod app;
pub mod core;
pub mod dto;
pub mod io;
pub mod nodes;
pub mod transport;
pub mod types;

fn main() {
    let app = Arc::new(Mutex::new(App::new()));
    tcp::run(app);
    MidiManager::print_available_input_ports();
    MidiManager::print_available_output_ports();
    let mut midi_manager = MidiManager::new();
    midi_manager.connect_input(3);
    loop {}
}
