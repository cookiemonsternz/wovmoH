use crate::core::node::*;
use crate::types::color::{Color, ColorValue};
use crate::types::data_type::*;
use std::ops::DerefMut;

fn merge_hsv_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let hue = match inputs[0] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };
    let saturation = match inputs[1] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };
    let value = match inputs[2] {
        DataValue::Number(number) => number,
        _ => panic!("Expected Number"),
    };

    match outputs[0].deref_mut() {
        DataValue::Color(color) => *color = Color::new(ColorValue::HSV(hue, saturation, value)),
        _ => panic!("Output is not color"),
    }
}

pub static MERGE_HSV_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Merge HSV",
    category: "Color",
    inputs: &[
        InputDesc {
            name: "H",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: Some(0.0),
                max: Some(1.0),
            },
        },
        InputDesc {
            name: "S",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: Some(0.0),
                max: Some(1.0),
            },
        },
        InputDesc {
            name: "V",
            data_type: DataType::Number,
            default: DataValue::default(DataType::Number),
            constraints: InputConstraints::Number {
                min: Some(0.0),
                max: Some(1.0),
            },
        },
    ],
    outputs: &[OutputDesc {
        name: "Color",
        data_type: DataType::Color,
    }],
    process: merge_hsv_process,
};
