use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn all(n: f32) -> Self {
        Vec3::new(n, n, n)
    }

    pub fn dot(&self, rhs: &Vec3) -> f32 {
        let pdt = self * rhs;
        pdt.x + pdt.y + pdt.z
    }

    pub fn len_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn normalized(&self) -> Self {
        self / self.len()
    }
}

impl_op_ex!(- |a: &Vec3| -> Vec3 { -1.0 * a });

impl_op_ex_commutative!(* |a: &Vec3, b: f32| -> Vec3 { Vec3::new(a.x * b, a.y * b, a.z * b) });
impl_op_ex!(/ |a: &Vec3, b: f32| -> Vec3 { Vec3::new(a.x / b, a.y / b, a.z / b) });

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x + b.x, a.y + b.y, a.z + b.z) });
impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x - b.x, a.y - b.y, a.z - b.z) });
impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x * b.x, a.y * b.y, a.z * b.z) });
impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3::new(a.x / b.x, a.y / b.y, a.z / b.z) });
