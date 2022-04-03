use std::f32::consts::FRAC_PI_4;

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::texture::{Solid, Texture};
use crate::util::map;

#[derive(Serialize, Deserialize)]
pub struct SkyBox {
    pub up: Texture,
    pub dn: Texture,
    pub lf: Texture,
    pub rt: Texture,
    pub ft: Texture,
    pub bk: Texture,
}

enum Dir {
    Up,
    Dn,
    Lf,
    Rt,
    Ft,
    Bk,
}

impl Dir {
    fn uv(&self, x: f32, y: f32, z: f32) -> (f32, f32) {
        let (tan_u, tan_v) = match self {
            Dir::Up => (x / y, -z / y),
            Dir::Dn => (x / -y, z / -y),
            Dir::Lf => (-z / -x, -y / -x),
            Dir::Rt => (z / x, -y / x),
            Dir::Ft => (x / -z, -y / -z),
            Dir::Bk => (-x / z, -y / z),
        };
        (
            map(tan_u.atan(), -FRAC_PI_4, FRAC_PI_4, 0.0, 1.0),
            map(tan_v.atan(), -FRAC_PI_4, FRAC_PI_4, 0.0, 1.0),
        )
    }
}

impl SkyBox {
    pub fn solid(color: Vec3) -> Self {
        SkyBox {
            up: Texture::Solid(Solid { color }),
            dn: Texture::Solid(Solid { color }),
            lf: Texture::Solid(Solid { color }),
            rt: Texture::Solid(Solid { color }),
            ft: Texture::Solid(Solid { color }),
            bk: Texture::Solid(Solid { color }),
        }
    }

    pub fn color(&self, dir: Vec3) -> Vec3 {
        let (x, y, z) = (dir.x, dir.y, dir.z);
        let dir = dir_of_max_abs_val(x, y, z);
        let tex = match dir {
            Dir::Up => &self.up,
            Dir::Dn => &self.dn,
            Dir::Lf => &self.lf,
            Dir::Rt => &self.rt,
            Dir::Ft => &self.ft,
            Dir::Bk => &self.bk,
        };
        let (u, v) = dir.uv(x, y, z);
        tex.color(u, v)
    }
}

fn dir_of_max_abs_val(x: f32, y: f32, z: f32) -> Dir {
    if x.abs() > y.abs() {
        if x.abs() > z.abs() {
            if x.is_sign_positive() {
                Dir::Rt
            } else {
                Dir::Lf
            }
        } else {
            if z.is_sign_positive() {
                Dir::Bk
            } else {
                Dir::Ft
            }
        }
    } else {
        if y.abs() > z.abs() {
            if y.is_sign_positive() {
                Dir::Up
            } else {
                Dir::Dn
            }
        } else {
            if z.is_sign_positive() {
                Dir::Bk
            } else {
                Dir::Ft
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uv_dir_works_in_front() {
        let top_lf = Vec3::new(-1.0, 1.0, -1.0).normalize();
        assert_eq!((0.0, 0.0), Dir::Ft.uv(top_lf.x, top_lf.y, top_lf.z));

        let top_rt = Vec3::new(1.0, 1.0, -1.0).normalize();
        assert_eq!((1.0, 0.0), Dir::Ft.uv(top_rt.x, top_rt.y, top_rt.z));

        let center = Vec3::new(0.0, 0.0, -1.0).normalize();
        assert_eq!((0.5, 0.5), Dir::Ft.uv(center.x, center.y, center.z));

        let bot_lf = Vec3::new(-1.0, -1.0, -1.0).normalize();
        assert_eq!((0.0, 1.0), Dir::Ft.uv(bot_lf.x, bot_lf.y, bot_lf.z));

        let bot_rt = Vec3::new(1.0, -1.0, -1.0).normalize();
        assert_eq!((1.0, 1.0), Dir::Ft.uv(bot_rt.x, bot_rt.y, bot_rt.z));
    }
}
