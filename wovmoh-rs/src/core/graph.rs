use std::collections::{HashMap, VecDeque};

use crate::{
    core::{input_field::*, node::*, output_pin::*},
    dto::{connection_dto::ConnectionDto, graph_dto::GraphDto},
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

    pub fn get_node_mut(&mut self, node_id: NodeId) -> &mut Node {
        &mut self.nodes[node_id]
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
                index: i as usize,
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
                index: i as usize,
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
        let node = self.nodes[node_id].clone();

        // Disconnect all connections
        for input_id in &node.inputs {
            let input = &self.inputs[*input_id];
            if input.connected_output.is_some() {
                self.disconnect(*input_id);
            }
        }

        for output_id in &node.outputs {
            let output = &self.outputs[*output_id];
            if output.connections.len() > 0 {
                for connected_input_id in output.connections.clone() {
                    self.disconnect(connected_input_id);
                }
            }
        }

        // Swap and remove all inputs
        for input_id in node.inputs {
            let last_input_id = self.inputs.len() - 1;

            if input_id != last_input_id {
                let swapped_input = &self.inputs[last_input_id];
                let parent = swapped_input.parent;

                let index = self.nodes[parent]
                    .inputs
                    .iter()
                    .position(|&id| id == last_input_id)
                    .unwrap();

                self.nodes[parent].inputs[index] = input_id;
            }

            self.inputs.swap_remove(input_id);
        }

        // Swap and remove all outputs
        for output_id in node.outputs {
            let last_output_id = self.outputs.len() - 1;

            if output_id != last_output_id {
                let swapped_output = &self.outputs[last_output_id];
                let parent = swapped_output.parent;

                let index = self.nodes[parent]
                    .outputs
                    .iter()
                    .position(|&id| id == last_output_id)
                    .unwrap();

                self.nodes[parent].outputs[index] = output_id;
            }

            self.outputs.swap_remove(output_id);
        }

        // Swap and remove node
        let last_node_id = self.nodes.len() - 1;
        self.nodes.swap_remove(node_id);

        if self.nodes.is_empty() {
            self.order_dirty = true;
            return;
        }

        if node_id != last_node_id {
            let node = &mut self.nodes[node_id];
            node.id = node_id;

            for &input_id in &node.inputs {
                self.inputs[input_id].parent = node_id;
            }

            for &output_id in &node.outputs {
                self.outputs[output_id].parent = node_id;
            }
        }

        self.order_dirty = true;
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

    pub fn input_id_for(&self, node_id: NodeId, field_index: usize) -> usize {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        node.inputs[field_index]
    }

    pub fn input_for(&self, node_id: NodeId, field_index: usize) -> &InputField {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        let id = node.inputs[field_index];

        &self.inputs[id]
    }

    pub fn input_for_mut(&mut self, node_id: NodeId, field_index: usize) -> &mut InputField {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        let id = node.inputs[field_index];

        &mut self.inputs[id]
    }

    pub fn inputs_for(&self, node_id: NodeId) -> Vec<&InputField> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        node.inputs.iter().map(|&id| &self.inputs[id]).collect()
    }

    pub fn inputs_for_mut(&mut self, node_id: NodeId) -> Vec<&mut InputField> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        // node.outputs
        //     .iter()
        //     .map(|&id| &mut self.outputs[id])
        //     .collect()
        // Hacky (still safe but in future use Vec.get_many_mut())
        let mut result = Vec::with_capacity(node.inputs.len());
        let inputs = self.inputs.as_mut_ptr();

        for &id in &node.inputs {
            unsafe {
                // Should be safe...
                result.push(&mut *inputs.add(id));
            }
        }

        result
    }

    pub fn output_id_for(&self, node_id: NodeId, pin_index: usize) -> usize {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        node.outputs[pin_index]
    }

    pub fn output_for(&self, node_id: NodeId, pin_index: usize) -> &OutputPin {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        let id = node.outputs[pin_index];

        &self.outputs[id]
    }

    pub fn output_for_mut(&mut self, node_id: NodeId, pin_index: usize) -> &mut OutputPin {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        let id = node.outputs[pin_index];

        &mut self.outputs[id]
    }

    pub fn outputs_for(&self, node_id: NodeId) -> Vec<&OutputPin> {
        let node = match self.nodes.iter().find(|&x| x.id == node_id) {
            Some(node) => node,
            None => panic!("Node not found in Graph"),
        };

        node.outputs.iter().map(|&id| &self.outputs[id]).collect()
    }

    pub fn outputs_for_mut(&mut self, node_id: NodeId) -> Vec<&mut OutputPin> {
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
                        from_node: self.outputs[output_pin_id].parent,
                        from_index: self.outputs[output_pin_id].index,
                        to_node: node.id,
                        to_index: self.inputs[*input_index].index,
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
    use crate::core::node;

    use super::*;

    #[test]
    fn test_graph_add_node() {
        let mut graph = Graph::new(0);

        let node_kind = node::NodeKind::ConstantNumber;
        let ui_state = node::NodeUIState::default();

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
        let ui_state = node::NodeUIState::default();

        graph.add_node(node_kind, ui_state);

        graph.remove_node(0);

        assert!(graph.has_node(0) == false);
    }

    #[test]
    fn test_graph_connect_disconnect_node() {
        let mut graph = Graph::new(0);

        let node_kind = node::NodeKind::ConstantNumber;
        let ui_state = node::NodeUIState::default();

        graph.add_node(node_kind, ui_state.clone());

        graph.add_node(node_kind, ui_state.clone());

        graph.connect(0, 1);

        graph.disconnect(1);
    }

    #[test]
    fn test_graph() {
        let mut graph = Graph::new(0);

        let node_kind = node::NodeKind::ConstantNumber;
        let ui_state = node::NodeUIState::default();

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
