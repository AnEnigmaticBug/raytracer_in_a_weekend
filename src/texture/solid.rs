use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Solid {
    pub color: Vec3,
}
