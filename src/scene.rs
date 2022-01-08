use std::fs::File;
use std::io::{Read, self};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::camera::Camera;
use crate::geometry::{Geometry, HitInfo};
use crate::primitive::{Ray3, Vec3};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub sky_color: Vec3,
    pub camera: Camera,
    pub items: Vec<Geometry>,
}

impl Scene {
    pub fn from_json<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(serde_json::from_str(&contents)?)
    }

    pub fn hit(&self, ray: &Ray3, tmin: f32, tmax: f32) -> Option<HitInfo> {
        let mut closest_hit: Option<HitInfo> = None;

        for hit in self
            .items
            .iter()
            .filter_map(|item| item.hit(ray, tmin, tmax))
        {
            closest_hit = match closest_hit {
                Some(closest_hit) if hit.t < closest_hit.t => Some(hit),
                None => Some(hit),
                _ => closest_hit,
            };
        }

        closest_hit
    }
}
