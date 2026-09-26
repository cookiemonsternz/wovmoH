use std::collections::{HashMap, VecDeque};

use crate::{
    core::{
        input_field::*,
        node::{self, *},
        output_pin::*,
    },
    dto::{
        connection_dto::ConnectionDto, graph_dto::GraphDto, input_field_dto::InputFieldDto,
        node_dto::NodeDto, output_pin_dto::OutputPinDto,
    },
    types::data_type::DataValue,
};

pub type GraphId = usize;

pub struct Graph {
    id: GraphId,

    nodes: Vec<Node>,
    inputs: Vec<InputField>,
    outputs: Vec<OutputPin>,

    nodes_map: HashMap<NodeId, usize>,
    execution_order: Vec<NodeId>,

    order_dirty: bool,
}

impl Graph {
    pub fn new(id: GraphId) -> Graph {
        Graph {
            id: id,
            nodes: Vec::new(),
            inputs: Vec::new(),
            outputs: Vec::new(),
            nodes_map: HashMap::new(),
            execution_order: Vec::new(),
            order_dirty: false,
        }
    }

    pub fn get_node(&self, node_id: NodeId) -> &Node {
        &self.nodes[node_id]
    }

    pub fn has_node(&self, node_id: NodeId) -> bool {
        return self.nodes.len() < node_id;
    }

    pub fn add_node(&mut self, kind: NodeKind, ui_state: NodeUIState) {
        let desc = kind.descriptor();

        let mut inputs = Vec::new();
        // Create Input Fields
        for (i, input_desc) in desc.inputs.iter().enumerate() {
            inputs.push(self.inputs.len());
            self.inputs.push(InputField {
                parent: self.nodes.len(),
                index: i as u8,
                value: input_desc.default,
                connected_output: None,
            });
        }

        let mut outputs = Vec::new();
        // Create Output Pins
        for (i, output_desc) in desc.outputs.iter().enumerate() {
            outputs.push(self.outputs.len());
            self.outputs.push(OutputPin {
                parent: self.nodes.len(),
                index: i as u8,
                value: DataValue::default(output_desc.data_type.clone()),
                connections: Vec::new(),
            });
        }

        // Create Node
        let node = Node {
            id: self.nodes.len(),
            kind,
            ui_state,
            inputs,
            outputs,
        };
        self.nodes.push(node);

        self.order_dirty = true;
    }

    pub fn remove_node(&mut self, node_id: NodeId) {
        // Swap and remove, then update node id.

        let node = self.nodes[node_id].clone();

        // Swap and remove all inputs
        for index in &node.inputs {
            // Get the parent of the input that will be swapped and change the parents reference to the input to
            // the current index
            let swapped_input_id = self.inputs.len() - 1;
            let swapped_input_parent = &mut self.nodes[self.inputs.last().unwrap().parent];
            let swapped_input_index = swapped_input_parent
                .inputs
                .iter()
                .find(|&x| *x == swapped_input_id)
                .unwrap()
                .clone();

            // Set the input id at the input to be swapped to the id to be swapped to
            swapped_input_parent.inputs[swapped_input_index] = *index;

            // Now swap
            self.inputs.swap_remove(*index);
        }

        // Swap and remove all outputs
        for index in node.outputs {
            // Get parent of output that will be swapped, and change the parents ref to the output to the current index.
            let swapped_output_id = self.outputs.len() - 1;
            let swapped_output_parent = &mut self.nodes[self.outputs.last().unwrap().parent];
            let swapped_output_index = swapped_output_parent
                .inputs
                .iter()
                .find(|&x| *x == swapped_output_id)
                .unwrap()
                .clone();

            swapped_output_parent.outputs[swapped_output_index] = index;

            // swap
            self.outputs.swap_remove(index);
        }

        // Swap and remove node
        self.nodes.swap_remove(node_id);

        if self.nodes.len() == 0 {
            self.order_dirty = true;
            return;
        }

        // Update node at node_id
        let node = &mut self.nodes[node_id];
        node.id = node_id;

        // Update all fields and pins parent ids
        for input_id in &node.inputs {
            let input_field = &mut self.inputs[*input_id];
            input_field.parent = node_id;
        }

        // Update all outputs
        for output_id in &node.outputs {
            let output_pin = &mut self.outputs[*output_id];
            output_pin.parent = node_id;
        }

        self.order_dirty = true
    }

    pub fn connect(&mut self, from: OutputId, to: InputId) {
        let input_field = &mut self.inputs[to];
        input_field.connected_output = Some(from);

        let output_pin = &mut self.outputs[from];
        output_pin.connections.push(to);

        self.order_dirty = true;
    }

    pub fn disconnect(&mut self, input_field_id: InputId) {
        let input_field = &mut self.inputs[input_field_id];

        match input_field.connected_output {
            Some(id) => id,
            None => panic!("Cannot disconnect, not connected!"),
        };
        input_field.connected_output = None;

        self.order_dirty = true;
    }

    fn inputs_for(&self, node_id: NodeId) -> Vec<&InputField> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        node.inputs.iter().map(|&id| &self.inputs[id]).collect()
    }

    fn outputs_for(&self, node_id: NodeId) -> Vec<&OutputPin> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        node.outputs.iter().map(|&id| &self.outputs[id]).collect()
    }

    fn outputs_for_mut(&mut self, node_id: NodeId) -> Vec<&mut OutputPin> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        // node.outputs
        //     .iter()
        //     .map(|&id| &mut self.outputs[id])
        //     .collect()
        // Hacky (still safe but in future use Vec.get_many_mut())
        let mut result = Vec::with_capacity(node.outputs.len());
        let outputs = self.outputs.as_mut_ptr();

        for &id in &node.outputs {
            unsafe {
                // Should be safe...
                result.push(&mut *outputs.add(id));
            }
        }

        result
    }

    fn input_values_for(&self, node_id: NodeId) -> Vec<DataValue> {
        self.inputs_for(node_id)
            .iter()
            .map(|input| match input.connected_output {
                Some(output_id) => self.outputs[output_id].value.clone(),
                None => input.value.clone(),
            })
            .collect()
    }

    fn output_values_for(&mut self, node_id: NodeId) -> Vec<&mut DataValue> {
        self.outputs_for_mut(node_id)
            .into_iter()
            .map(|output| &mut output.value)
            .collect()
    }

    fn calculate_indegrees(&self) -> HashMap<NodeId, u32> {
        let mut indegrees = HashMap::with_capacity(self.nodes.len());
        for input in &self.inputs {
            indegrees.insert(input.parent, 0);
        }

        for input in &self.inputs {
            if input.connected_output.is_some() {
                let indegrees_val = indegrees.get_mut(&input.parent).unwrap();
                *indegrees_val += 1;
            }
        }

        indegrees
    }

    fn sort_nodes(&mut self) {
        let mut in_degrees = self.calculate_indegrees();
        let mut queue: VecDeque<NodeId> = VecDeque::new();

        for (id, in_degree) in in_degrees.iter() {
            if *in_degree == 0 {
                queue.push_back(*id);
            }
        }

        self.execution_order.clear();

        while !queue.is_empty() {
            let node_id = queue.pop_front().unwrap();

            self.execution_order.push(node_id);

            // Add connections to queue
            for output in self.outputs_for(node_id) {
                for input_id in &output.connections {
                    let input_field = &self.inputs[*input_id];
                    // let node = self.nodes[input_field.parent];

                    in_degrees.entry(input_field.parent).and_modify(|x| *x -= 1);

                    if in_degrees[&input_field.parent] == 0 {
                        queue.push_back(input_field.parent);
                    }
                }
            }
        }

        // Check for loops
        if self.execution_order.len() != self.nodes.len() {
            panic!("Loop in graph");
        }

        self.order_dirty = false;
    }

    pub fn process(&mut self) {
        if self.order_dirty {
            self.sort_nodes()
        };

        for node_id in &self.execution_order.clone() {
            let node = &self.nodes[*node_id].clone();

            let inputs = self.input_values_for(*node_id);
            let mut outputs = self.output_values_for(*node_id);

            (node.kind.descriptor().process)(inputs, outputs.as_mut());
        }
    }

    pub fn to_dto(&self) -> GraphDto {
        let mut connections: Vec<ConnectionDto> = Vec::new();

        for node in &self.nodes {
            for input_index in &node.inputs {
                match self.inputs[*input_index].connected_output {
                    Some(output_pin_id) => connections.push(ConnectionDto {
                        from: output_pin_id,
                        to: *input_index,
                    }),
                    None => continue,
                };
            }
        }

        GraphDto {
            id: self.id,
            nodes: self.nodes.iter().map(|x| x.to_dto()).collect(),
            connections: connections,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_add_node() {
        let mut graph = Graph::new(0);

        let node_kind = node::NodeKind::ConstantNumber;
        let ui_state = node::NodeUIState {
            position: (0.0, 0.0),
        };

        graph.add_node(node_kind, ui_state);

        let node = graph.get_node(0);

        // assert!(node.kind. == node_kind);
        // assert!(node.ui_state == ui_state);
        assert!(node.id == 0);
    }

    #[test]
    fn test_graph_remove_node() {
        let mut graph = Graph::new(0);

        let node_kind = node::NodeKind::ConstantNumber;
        let ui_state = node::NodeUIState {
            position: (0.0, 0.0),
        };

        graph.add_node(node_kind, ui_state);

        graph.remove_node(0);

        assert!(graph.has_node(0) == false);
    }

    #[test]
    fn test_graph_connect_disconnect_node() {
        let mut graph = Graph::new(0);

        let node_kind = node::NodeKind::ConstantNumber;
        let ui_state = node::NodeUIState {
            position: (0.0, 0.0),
        };

        graph.add_node(node_kind, ui_state.clone());

        graph.add_node(node_kind, ui_state.clone());

        graph.connect(0, 1);

        graph.disconnect(1);
    }

    #[test]
    fn test_graph() {
        let mut graph = Graph::new(0);

        let node_kind = node::NodeKind::ConstantNumber;
        let ui_state = node::NodeUIState {
            position: (0.0, 0.0),
        };

        graph.add_node(node_kind, ui_state.clone());
        graph.add_node(node_kind, ui_state.clone());

        graph.connect(0, 1);

        // for mut output_value in graph.output_values_for(0) {
        //     output_value = &mut DataValue::Number(5.0);
        // }

        graph.inputs[0].value = DataValue::Number(5.0);

        // graph.sort_nodes();

        graph.process();

        assert!(graph.outputs[1].value == DataValue::Number(5.0))
    }
}
