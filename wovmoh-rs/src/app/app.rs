use crate::core::graph_manager::GraphManager;

pub struct App {
    pub graphs: GraphManager,
}

impl App {
    pub fn new() -> App {
        App {
            graphs: GraphManager::new(),
        }
    }
}
