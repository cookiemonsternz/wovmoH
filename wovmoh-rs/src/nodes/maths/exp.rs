use std::ops::DerefMut;

use crate::core::node::*;
use crate::types::data_type::*;

fn exp_numbers_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_number_a = match inputs[0] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };
    let input_number_b = match inputs[1] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };

    match outputs[0].deref_mut() {
        DataValue::Number(n) => *n = input_number_a.powf(input_number_b),
        _ => panic!("Output is not number"),
    }
}

pub static EXP_NUMBERS_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Exp",
    category: "Maths",
    inputs: &[
        InputDesc {
            name: "Base",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: None,
                max: None,
            },
        },
        InputDesc {
            name: "Exponent",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: None,
                max: None,
            },
        },
    ],
    outputs: &[OutputDesc {
        name: "Result",
        data_type: DataType::Number,
    }],
    process: exp_numbers_process,
};
