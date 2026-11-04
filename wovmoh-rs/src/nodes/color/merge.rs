use crate::core::node::*;
use crate::types::color::Color;
use crate::types::data_type::*;
use std::ops::DerefMut;

fn merge_color_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let r = match inputs[0] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };
    let g = match inputs[1] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };
    let b = match inputs[2] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };

    match outputs[0].deref_mut() {
        DataValue::Color(n) => *n = Color::new(crate::types::color::ColorValue::RGB(r, g, b)),
        _ => panic!("Output is not color"),
    }
}

pub static MERGE_COLOR_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Merge Color",
    category: "Color",
    inputs: &[
        InputDesc {
            name: "R",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: Some(0f64),
                max: Some(1f64),
            },
        },
        InputDesc {
            name: "G",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: Some(0f64),
                max: Some(1f64),
            },
        },
        InputDesc {
            name: "B",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: Some(0f64),
                max: Some(1f64),
            },
        },
    ],
    outputs: &[OutputDesc {
        name: "Color",
        data_type: DataType::Color,
    }],
    process: merge_color_process,
};
