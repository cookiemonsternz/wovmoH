use crate::core::node::*;
use crate::types::data_type::*;
use std::ops::DerefMut;

fn split_hsv_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_color = match inputs[0] {
        DataValue::Color(color) => color,
        _ => panic!("Expected Color"),
    };
    let (hue, saturation, value) = input_color.hsv();

    for (output, channel) in outputs.iter_mut().zip([hue, saturation, value]) {
        match output.deref_mut() {
            DataValue::Number(number) => *number = channel,
            _ => panic!("Output is not number"),
        }
    }
}

pub static SPLIT_HSV_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Split HSV",
    category: "Color",
    inputs: &[InputDesc {
        name: "Color",
        data_type: DataType::Color,
        default: DataValue::default(DataType::Color),
        constraints: InputConstraints::Color,
    }],
    outputs: &[
        OutputDesc {
            name: "H",
            data_type: DataType::Number,
        },
        OutputDesc {
            name: "S",
            data_type: DataType::Number,
        },
        OutputDesc {
            name: "V",
            data_type: DataType::Number,
        },
    ],
    process: split_hsv_process,
};
