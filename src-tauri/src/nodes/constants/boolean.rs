use crate::core::node::*;
use crate::types::data_type::*;

fn constant_boolean_process(inputs: Vec<DataValue>, outputs: &mut Vec<&mut DataValue>) {
    let input_bool = match inputs[0] {
        DataValue::Boolean(boolean) => boolean,
        _ => panic!("Expected Boolean"),
    };

    if let DataValue::Boolean(ref mut n) = outputs[0] {
        *n = input_bool;
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