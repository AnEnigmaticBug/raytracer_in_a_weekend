use glam::Vec3;

#[derive(clap::ArgEnum, Clone)]
pub enum ToneMapper {
    /// Simply clamps light values to a 0-1 range.
    Clamp,
    /// John Hable's Uncharted 2 tone-mapper. Here's his informative blog post:
    /// http://filmicworlds.com/blog/filmic-tonemapping-operators/
    Uncharted,
}

impl ToneMapper {
    pub fn map(&self, color: Vec3) -> Vec3 {
        match self {
            Self::Clamp => color.clamp(Vec3::ZERO, Vec3::ONE),
            Self::Uncharted => uncharted_tone_map(color),
        }
    }
}

fn uncharted_tone_map_partial(color: Vec3) -> Vec3 {
    /// Lots of magic constants for magical results.
    const A: f32 = 0.15;
    const B: f32 = 0.50;
    const C: f32 = 0.10;
    const D: f32 = 0.20;
    const E: f32 = 0.02;
    const F: f32 = 0.30;

    ((color * (A * color + C * B) + D * E) / (color * (A * color + B) + D * F)) - Vec3::splat(E / F)
}

fn uncharted_tone_map(color: Vec3) -> Vec3 {
    const EXPOSURE_BIAS: f32 = 2.0;
    let w: Vec3 = Vec3::splat(11.2);
    let white_scale = 1.0 / uncharted_tone_map_partial(w);
    uncharted_tone_map_partial(color * EXPOSURE_BIAS) * white_scale
}
