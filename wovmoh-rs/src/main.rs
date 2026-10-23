use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

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

    let timer = timer::Timer::new();
    let _guard = timer.schedule_repeating(chrono::Duration::milliseconds(16), move || {
        // println!("Timer Fired");
        match app.try_lock() {
            Ok(mut app_guard) => {
                app_guard.event_loop();
            }
            Err(_) => {
                println!("App mutex is locked")
            }
        }
    });

    std::thread::park();
}
