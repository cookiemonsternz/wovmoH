use serde::{Deserialize, Serialize};

use crate::core::graph::Graph;
use crate::dto::input_field_dto::InputFieldDto;
use crate::dto::node_dto::NodeDto;
use crate::dto::output_pin_dto::OutputPinDto;
use crate::nodes;
use crate::types::data_type::{DataType, DataValue};

pub type NodeId = usize;

#[derive(Clone)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub ui_state: NodeUIState,
    pub inputs: Vec<usize>,
    pub outputs: Vec<usize>,
}

impl Node {
    pub fn to_dto(&self) -> NodeDto {
        NodeDto {
            id: self.id,
            name: self.kind.descriptor().name,
            kind: self.kind.into(),
            position: self.ui_state.position,
            name_override: self.ui_state.name_override.clone(),
            inputs: self
                .kind
                .descriptor()
                .inputs
                .iter()
                .map(|x| InputFieldDto {
                    name: x.name,
                    data_type: x.data_type,
                    value: DataValue::default(x.data_type),
                    constraints: x.constraints,
                })
                .collect(),
            outputs: self
                .kind
                .descriptor()
                .outputs
                .iter()
                .map(|x| OutputPinDto {
                    name: x.name,
                    data_type: x.data_type,
                    value: DataValue::default(x.data_type),
                })
                .collect(),
        }
    }
}

#[derive(Copy, Clone, Deserialize, strum_macros::IntoStaticStr, strum_macros::EnumIter)]
pub enum NodeKind {
    ConstantNumber,
    ConstantBoolean,
    ConstantColor,
}

impl NodeKind {
    pub fn descriptor(&self) -> &'static NodeDescriptor {
        match self {
            NodeKind::ConstantNumber => &nodes::constants::number::CONSTANT_NUMBER_DESCRIPTOR,
            NodeKind::ConstantBoolean => &nodes::constants::boolean::CONSTANT_BOOLEAN_DESCRIPTOR,
            NodeKind::ConstantColor => &nodes::constants::color::CONSTANT_COLOR_DESCRIPTOR,
        }
    }
}

pub struct NodeDescriptor {
    pub name: &'static str,
    pub inputs: &'static [InputDesc],
    pub outputs: &'static [OutputDesc],
    pub process: fn(Vec<DataValue>, &mut Vec<&mut DataValue>),
}

pub struct InputDesc {
    pub name: &'static str,
    pub data_type: DataType,
    pub default: DataValue,
    pub constraints: InputConstraints,
}

#[derive(Serialize, Debug, Clone, Copy)]
#[serde(tag = "type", content = "value")]
pub enum InputConstraints {
    Number { min: Option<f64>, max: Option<f64> },
    Boolean,
    Color,
    Vector3,
    Point3,
}

#[derive(Clone)]
pub struct OutputDesc {
    pub name: &'static str,
    pub data_type: DataType,
}

#[derive(Clone, Default)]
pub struct NodeUIState {
    pub position: (f64, f64),
    pub name_override: String,
}

// use crate::core::graph::Graph;

// use super::super::types::data_type::DataValue;
// use super::pin::*;

// pub struct NodeProperties {
//     pub id: usize,
// }

// pub trait Node {
//     fn properties(&self) -> &NodeProperties;
//     fn input_fields(&self) -> Vec<usize>;
//     fn output_pins(&self) -> Vec<usize>;

//     fn eq(&self, other: &Box<dyn Node>) -> bool {
//         self.properties().id == other.properties().id
//     }

//     // fn get_input_field(&self, id: usize) -> &InputField {
//     //     self.input_fields()
//     //         .into_iter()
//     //         .find(|x| x.id() == id)
//     //         .unwrap()
//     // }

//     // fn get_output_pin(&self, id: usize) -> &OutputPin {
//     //     self.output_pins()
//     //         .into_iter()
//     //         .find(|x| x.id() == id)
//     //         .unwrap()
//     // }

//     // fn add_input_field(&mut self, id: usize, value: DataValue) {
//     //     let input_field = InputField::new(id, self.properties().id, value);

//     //     self.input_fields().push(input_field);
//     // }

//     // fn add_output_pin(&mut self, id: usize) {
//     //     let output_pin = OutputPin::new(id, self.properties().id);

//     //     self.output_pins().push(output_pin);
//     // }

//     fn get_in_degree(&self, graph: &Graph) -> usize {
//         let mut in_degree = 0;

//         for input_field_id in self.input_fields() {
//             let input_field = graph.get_input_field(&input_field_id);
//             if input_field.is_connected() {
//                 in_degree += 1;
//             }
//         }

//         in_degree
//     }

//     // Updates output pin values based on input field values
//     fn process(&self);
// }
