// use crate::core;
// use crate::types;

// use core::node;
// use core::pin;
// use types::data_type;
// use types::vector;

// pub struct ConstantVectorNode {
//     properties: node::NodeProperties,
//     input_fields: Vec<pin::InputField>,
//     output_pins: Vec<pin::OutputPin>,
// }

// impl ConstantVectorNode {
//     pub fn new(id: i32, vector: vector::Vec3) {
//         let node = ConstantVectorNode {
//             properties: node::NodeProperties { id },
//             input_fields: Vec::new(),
//             output_pins: Vec::new(),
//         };

//         // Main Field - Color
//         node.add_input_field(0, data_type::DataValue::Vector3(vector));
//         // Main Output Pin
//         node.add_output_pin(0);
//     }
// }

// impl node::Node for ConstantVectorNode {
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
//         self.get_output_pin(0)
//             .set_value(self.get_input_field(0).value);
//     }
// }
