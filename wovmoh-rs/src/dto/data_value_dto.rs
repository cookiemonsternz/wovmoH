use crate::types::*;
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(tag = "type", content = "value")]
pub enum DataValueDto {
    Number(f64),
    Boolean(bool),
    Color(color::Color),
    Vector3(vector::Vec3),
    Point3(vector::Point3),
}
