use std::f32::consts::FRAC_PI_4;

use glam::Vec3;
use serde::{Deserialize, Serialize};

use crate::cache::Cache;
use crate::texture::Texture;
use crate::util::map;

#[derive(Serialize, Deserialize)]
pub struct Cubemap {
    pub up_idx: usize,
    pub dn_idx: usize,
    pub lf_idx: usize,
    pub rt_idx: usize,
    pub ft_idx: usize,
    pub bk_idx: usize,
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

impl Cubemap {
    pub fn all(tex_idx: usize) -> Self {
        Cubemap {
            up_idx: tex_idx,
            dn_idx: tex_idx,
            lf_idx: tex_idx,
            rt_idx: tex_idx,
            ft_idx: tex_idx,
            bk_idx: tex_idx,
        }
    }

    pub fn color(&self, texture_cache: &Cache<Texture>, dir: Vec3) -> Vec3 {
        let (x, y, z) = (dir.x, dir.y, dir.z);
        let dir = dir_of_max_abs_val(x, y, z);
        let tex_idx = match dir {
            Dir::Up => self.up_idx,
            Dir::Dn => self.dn_idx,
            Dir::Lf => self.lf_idx,
            Dir::Rt => self.rt_idx,
            Dir::Ft => self.ft_idx,
            Dir::Bk => self.bk_idx,
        };
        let (u, v) = dir.uv(x, y, z);
        texture_cache[tex_idx].color(u, v)
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
