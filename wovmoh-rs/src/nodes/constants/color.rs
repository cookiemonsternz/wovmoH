use crate::core::node::*;
use crate::types::color::Color;
use crate::types::data_type::*;
use std::ops::DerefMut;

fn constant_color_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_color = match inputs[0] {
        DataValue::Color(color) => color,
        _ => panic!("Expected Color"),
    };

    match outputs[0].deref_mut() {
        DataValue::Color(n) => *n = input_color,
        _ => panic!("Output is not color"),
    }
}

pub static CONSTANT_COLOR_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Constant Color",
    inputs: &[InputDesc {
        id: 0,
        name: "Color",
        data_type: DataType::Color,
        default: DataValue::Color(Color::default()),
    }],
    outputs: &[OutputDesc {
        id: 0,
        name: "Color",
        data_type: DataType::Color,
    }],
    process: constant_color_process,
};
