use std::ops::DerefMut;

use crate::core::node::*;
use crate::types::data_type::*;

fn add_numbers_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_number_a = match inputs[0] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };
    let input_number_b = match inputs[1] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };

    match outputs[0].deref_mut() {
        DataValue::Number(n) => *n = input_number_a + input_number_b,
        _ => panic!("Output is not number"),
    }
}

pub static ADD_NUMBERS_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Add",
    inputs: &[
        InputDesc {
            name: "Number A",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: None,
                max: None,
            },
        },
        InputDesc {
            name: "Number B",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: None,
                max: None,
            },
        },
    ],
    outputs: &[OutputDesc {
        name: "Sum",
        data_type: DataType::Number,
    }],
    process: add_numbers_process,
};
