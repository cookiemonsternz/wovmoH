use crate::{core::graph_manager::GraphManager, io::midi::MidiManager};

pub struct App {
    pub graphs: GraphManager,
    pub midi: MidiManager,
}

impl App {
    pub fn new() -> App {
        App {
            graphs: GraphManager::new(),
        }
    }
}
