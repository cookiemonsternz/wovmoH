use std::ops::DerefMut;

use crate::core::node::*;
use crate::types::data_type::*;

fn constant_number_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_number = match inputs[0] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };

    match outputs[0].deref_mut() {
        DataValue::Number(n) => *n = input_number,
        _ => panic!("Output is not number"),
    }
}

pub static CONSTANT_NUMBER_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Constant Number",
    inputs: &[InputDesc {
        name: "Number",
        data_type: DataType::Number,
        default: DataValue::default(DataType::Number),
        constraints: InputConstraints::Number {
            min: None,
            max: None,
        },
    }],
    outputs: &[OutputDesc {
        name: "Number",
        data_type: DataType::Number,
    }],
    process: constant_number_process,
};

// use crate::core;
// use crate::types;

// use core::node;
// use core::pin;
// use types::data_type;

// pub struct ConstantNumberNode {
//     properties: node::NodeProperties,
//     input_fields: Vec<pin::InputField>,
//     output_pins: Vec<pin::OutputPin>,
// }

// impl ConstantNumberNode {
//     pub fn new(id: i32, number: f64) {
//         let node = ConstantNumberNode {
//             properties: node::NodeProperties { id },
//             input_fields: Vec::new(),
//             output_pins: Vec::new(),
//         };

//         // Main Field - Color
//         node.add_input_field(0, data_type::DataValue::Number(number));
//         // Main Output Pin
//         node.add_output_pin(0);
//     }
// }

// impl node::Node for ConstantNumberNode {
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
