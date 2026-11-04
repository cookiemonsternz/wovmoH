use crate::core::node::*;
use crate::types::data_type::*;
use std::ops::DerefMut;

fn split_color_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_color = match inputs[0] {
        DataValue::Color(color) => color,
        _ => panic!("Expected Color"),
    };

    match outputs[0].deref_mut() {
        DataValue::Number(n) => *n = input_color.r(),
        _ => panic!("Output is not number"),
    }
    match outputs[1].deref_mut() {
        DataValue::Number(n) => *n = input_color.g(),
        _ => panic!("Output is not number"),
    }
    match outputs[2].deref_mut() {
        DataValue::Number(n) => *n = input_color.b(),
        _ => panic!("Output is not number"),
    }
}

pub static SPLIT_COLOR_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Split Color",
    category: "Color",
    inputs: &[InputDesc {
        name: "Color",
        data_type: DataType::Color,
        default: DataValue::default(DataType::Color),
        constraints: InputConstraints::Color,
    }],
    outputs: &[
        OutputDesc {
            name: "R",
            data_type: DataType::Number,
        },
        OutputDesc {
            name: "G",
            data_type: DataType::Number,
        },
        OutputDesc {
            name: "B",
            data_type: DataType::Number,
        },
    ],
    process: split_color_process,
};
