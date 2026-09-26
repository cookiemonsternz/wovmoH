use std::ops::DerefMut;

use crate::core::node::*;
use crate::types::data_type::*;

fn constant_boolean_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_bool = match inputs[0] {
        DataValue::Boolean(boolean) => boolean,
        _ => panic!("Expected Boolean"),
    };

    match outputs[0].deref_mut() {
        DataValue::Boolean(n) => *n = input_bool,
        _ => panic!("Output is not boolean"),
    };
}

pub static CONSTANT_BOOLEAN_DESCRIPTOR: NodeDescriptor = NodeDescriptor {
    name: "Constant Boolean",
    inputs: &[InputDesc {
        id: 0,
        name: "Boolean",
        data_type: DataType::Boolean,
        default: DataValue::default(DataType::Boolean),
    }],
    outputs: &[OutputDesc {
        id: 0,
        name: "Boolean",
        data_type: DataType::Boolean,
    }],
    process: constant_boolean_process,
};
