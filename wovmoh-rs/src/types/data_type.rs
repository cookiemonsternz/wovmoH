use super::*;
use serde::Serialize;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Serialize)]
pub enum DataType {
    Number,
    Boolean,
    Color,
    Vector3,
    Point3,
}

#[derive(Copy, Clone, Serialize, Debug)]
#[serde(tag = "type", content = "value")]
pub enum DataValue {
    Number(f64),
    Boolean(bool),
    Color(color::Color),
    Vector3(vector::Vec3),
    Point3(vector::Point3),
}

impl DataValue {
    // Used for errors
    pub fn type_name(&self) -> &'static str {
        match self {
            DataValue::Number(_) => "Number",
            DataValue::Boolean(_) => "Boolean",
            DataValue::Color(_) => "Color",
            DataValue::Vector3(_) => "Vector3",
            DataValue::Point3(_) => "Point3",
        }
    }

    pub const fn default(data_type: DataType) -> DataValue {
        match data_type {
            DataType::Number => DataValue::Number(0.0),
            DataType::Boolean => DataValue::Boolean(false),
            DataType::Color => DataValue::Color(color::Color::default()),
            DataType::Vector3 => DataValue::Vector3(vector::Vec3::default()),
            DataType::Point3 => DataValue::Point3(vector::Point3::default()),
        }
    }
}

// Error type for DataValue ops
pub enum EvalError {
    TypeError {
        op: &'static str,
        lhs: &'static str,
        rhs: &'static str,
    },
    UnsupportedOp {
        op: &'static str,
        lhs: &'static str,
        rhs: &'static str,
    },
}

impl Add for DataValue {
    type Output = Result<DataValue, EvalError>;

    fn add(self, rhs: DataValue) -> Self::Output {
        use DataValue::*;

        match (self, rhs) {
            (Number(a), Number(b)) => Ok(Number(a + b)),

            (Color(a), Color(b)) => Ok(Color(a + b)),

            (Vector3(a), Vector3(b)) => Ok(Vector3(a + b)),
            (Point3(a), Point3(b)) => Ok(Point3(a + b)),

            (Boolean(_), _) | (_, Boolean(_)) => Err(EvalError::TypeError {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),

            _ => Err(EvalError::UnsupportedOp {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),
        }
    }
}

impl Sub for DataValue {
    type Output = Result<DataValue, EvalError>;

    fn sub(self, rhs: DataValue) -> Self::Output {
        use DataValue::*;

        match (self, rhs) {
            (Number(a), Number(b)) => Ok(Number(a - b)),

            (Color(a), Color(b)) => Ok(Color(a - b)),

            (Vector3(a), Vector3(b)) => Ok(Vector3(a - b)),
            (Point3(a), Point3(b)) => Ok(Point3(a - b)),

            (Boolean(_), _) | (_, Boolean(_)) => Err(EvalError::TypeError {
                op: "-",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),

            _ => Err(EvalError::UnsupportedOp {
                op: "-",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),
        }
    }
}

impl Mul for DataValue {
    type Output = Result<DataValue, EvalError>;

    fn mul(self, rhs: DataValue) -> Self::Output {
        use DataValue::*;

        match (self, rhs) {
            (Number(a), Number(b)) => Ok(Number(a * b)),

            (Color(a), Color(b)) => Ok(Color(a * b)),

            (Vector3(a), Vector3(b)) => Ok(Vector3(a * b)),
            (Point3(a), Point3(b)) => Ok(Point3(a * b)),

            (Boolean(_), _) | (_, Boolean(_)) => Err(EvalError::TypeError {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),

            _ => Err(EvalError::UnsupportedOp {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),
        }
    }
}

impl Div for DataValue {
    type Output = Result<DataValue, EvalError>;

    fn div(self, rhs: DataValue) -> Self::Output {
        use DataValue::*;

        match (self, rhs) {
            (Number(a), Number(b)) => Ok(Number(a / b)),

            (Color(a), Color(b)) => Ok(Color(a / b)),

            (Vector3(a), Vector3(b)) => Ok(Vector3(a / b)),
            (Point3(a), Point3(b)) => Ok(Point3(a / b)),

            (Boolean(_), _) | (_, Boolean(_)) => Err(EvalError::TypeError {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),

            _ => Err(EvalError::UnsupportedOp {
                op: "+",
                lhs: self.type_name(),
                rhs: rhs.type_name(),
            }),
        }
    }
}

impl PartialEq for DataValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::Color(l0), Self::Color(r0)) => l0 == r0,
            (Self::Vector3(l0), Self::Vector3(r0)) => l0 == r0,
            (Self::Point3(l0), Self::Point3(r0)) => l0 == r0,
            _ => false,
        }
    }
}
