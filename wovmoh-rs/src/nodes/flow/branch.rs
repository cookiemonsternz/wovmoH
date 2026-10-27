use std::ops::DerefMut;

use crate::core::node::*;
use crate::types::data_type::*;

fn branch_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let branch_input = match inputs[0] {
        DataValue::Boolean(value) => value,
        _ => panic!("Expected Bool"),
    };
    let input_number_a = match inputs[1] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };
    let input_number_b = match inputs[2] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };

    match outputs[0].deref_mut() {
        DataValue::Number(n) => {
            *n = if branch_input {
                input_number_a
            } else {
                input_number_b
            }
        }
        _ => panic!("Output is not number"),
    }
}

pub static BRANCH_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Branch",
    category: "Logic",
    inputs: &[
        InputDesc {
            name: "Branch",
            data_type: DataType::Boolean,
            default: DataValue::default(DataType::Boolean),
            constraints: InputConstraints::Boolean,
        },
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
        name: "Selected",
        data_type: DataType::Number,
    }],
    process: branch_process,
};
