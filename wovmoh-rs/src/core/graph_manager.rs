use crate::{core::graph::*, io::midi::MidiEvent};

pub struct GraphManager {
    graphs: Vec<Graph>,
}

impl GraphManager {
    pub fn new() -> Self {
        GraphManager { graphs: Vec::new() }
    }

    pub fn add_graph(&mut self) -> GraphId {
        self.graphs.push(Graph::new(self.graphs.len()));
        self.graphs.len() - 1
    }

    pub fn get_graph(&self, id: GraphId) -> &Graph {
        &self.graphs[id]
    }

    pub fn get_graph_mut(&mut self, id: GraphId) -> &mut Graph {
        &mut self.graphs[id]
    }

    pub fn handle_midi_event(&mut self, event: MidiEvent) {
        let graph = self.get_graph_mut(event.graph);
        graph.update_midi_input(event.field, &event.message);
    }
}
