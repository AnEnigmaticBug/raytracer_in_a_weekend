use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub geometry_idx: usize,
    pub material_idx: usize,
}
