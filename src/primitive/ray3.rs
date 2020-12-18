use super::Vec3;

pub struct Ray3 {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray3 {
    pub fn new(pos: Vec3, dir: Vec3) -> Self {
        Ray3 { pos, dir }
    }

    pub fn point_at_param(&self, t: f32) -> Vec3 {
        self.pos + self.dir * t
    }
}
