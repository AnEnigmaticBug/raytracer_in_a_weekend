use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LinearGradient {
    pub from: Vec3,
    pub to: Vec3,
}

impl LinearGradient {
    pub fn color(&self, u: f32) -> Vec3 {
        self.from * (1.0 - u) + self.to * u
    }
}
