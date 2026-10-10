use std::sync::{Arc, Mutex};

use crate::{app::App, io::midi::MidiManager, transport::tcp};

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
    tcp::run(app.clone());
    loop {
        {
            let mut app_guard = app.lock().expect("App mutex poisoned");
            app_guard.process_midi_events();
        }
    }
}
