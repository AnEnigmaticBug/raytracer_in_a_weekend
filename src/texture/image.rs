use std::convert::TryFrom;
use std::path::Path;

use glam::Vec3;
use image::{io::Reader, DynamicImage, GenericImageView, ImageError, ImageResult};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct TexturePath {
    path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(try_from = "TexturePath")]
pub struct Image {
    pub path: String,
    #[serde(skip_serializing)]
    image: DynamicImage,
}

impl Image {
    pub fn load<P: AsRef<Path>>(path: P) -> ImageResult<Self> {
        let image = Reader::open(&path)?.decode()?;
        let channels = image.color().channel_count() as usize;

        assert!(channels == 3 || channels == 4);

        Ok(Image {
            path: path.as_ref().to_str().expect("Non UTF-8 path").to_owned(),
            image,
        })
    }

    pub fn color(&self, u: f32, v: f32) -> Vec3 {
        let (wd, ht) = self.image.dimensions();
        let x = (u * (wd - 1) as f32) as u32;
        let y = (v * (ht - 1) as f32) as u32;
        let [r, g, b, _] = self.image.get_pixel(x, y).0;
        Vec3::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
    }
}

impl TryFrom<TexturePath> for Image {
    type Error = ImageError;

    fn try_from(path: TexturePath) -> Result<Self, Self::Error> {
        Image::load(path.path)
    }
}
