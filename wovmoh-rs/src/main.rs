use std::sync::{Arc, Mutex};

use crate::{app::App, transport::tcp};

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

#[cfg(test)]
mod tests {
    use std::{
        ops::AddAssign,
        sync::{Arc, Mutex, RwLock},
    };

    use crate::{app::App, transport::tcp};

    #[test]
    fn main() {
        let app = Arc::new(Mutex::new(App::new()));
        tcp::run(app.clone());

        let count = Arc::new(RwLock::new(0));

        let timer = timer::Timer::new();
        let count_clone = count.clone();
        let _guard = timer.schedule_repeating(chrono::Duration::milliseconds(16), move || {
            // println!("Timer Fired");
            match app.try_lock() {
                Ok(mut app_guard) => {
                    app_guard.event_loop();
                }
                Err(_) => {
                    eprintln!("App mutex is locked")
                }
            }

            count_clone.write().unwrap().add_assign(1);
        });

        loop {
            if count.read().unwrap().gt(&10i32) {
                return;
            }
        }
    }
}
