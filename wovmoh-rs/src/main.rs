use std::sync::{Arc, Mutex};

use crate::{app::App, transport::tcp};

pub mod api;
pub mod app;
pub mod core;
pub mod dto;
pub mod nodes;
pub mod transport;
pub mod types;

fn main() {
    let app = Arc::new(Mutex::new(App::new()));
    tcp::run(app);
    loop {}
}
