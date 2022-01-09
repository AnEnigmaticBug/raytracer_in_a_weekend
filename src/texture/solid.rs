use serde::{Deserialize, Serialize};

use crate::primitive::Vec3;

#[derive(Serialize, Deserialize)]
pub struct Solid {
    pub color: Vec3,
}
