// use crate::core;
// use crate::types;

// use core::node;
// use core::pin;
// use types::data_type;

// pub struct AddNumberNode {
//     properties: node::NodeProperties,
//     input_fields: Vec<pin::InputField>,
//     output_pins: Vec<pin::OutputPin>,
// }

// impl AddNumberNode {
//     pub fn new(id: i32, num_a: f64, num_b: f64) {
//         let node = AddNumberNode {
//             properties: node::NodeProperties { id },
//             input_fields: Vec::new(),
//             output_pins: Vec::new(),
//         };

//         // Main Field - Color
//         node.add_input_field(0, data_type::DataValue::Number(num_a));
//         node.add_input_field(1, data_type::DataValue::Number(num_b));
//         // Main Output Pin
//         node.add_output_pin(0);
//     }
// }

// impl node::Node for AddNumberNode {
//     fn properties(&self) -> &node::NodeProperties {
//         &self.properties
//     }

//     fn input_fields(&self) -> &Vec<pin::InputField> {
//         &self.input_fields
//     }

//     fn output_pins(&self) -> &Vec<pin::OutputPin> {
//         &self.output_pins
//     }

//     // Output value = input value;
//     fn process(&self) {
//         let sum = self.get_input_field(0).value + self.get_input_field(1).value;
//         self.get_output_pin(0).set_value(sum);
//     }
// }
