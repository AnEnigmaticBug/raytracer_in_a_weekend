use std::convert::TryFrom;
use std::fs::File;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::primitive::Vec3;

#[derive(Deserialize)]
struct TexturePath {
    path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(try_from = "TexturePath")]
pub struct Image {
    pub path: String,
    #[serde(skip_serializing)]
    pub buf: Vec<u8>,
    #[serde(skip_serializing)]
    pub wd: usize,
    #[serde(skip_serializing)]
    pub ht: usize,
    #[serde(skip_serializing)]
    pub channels: usize,
}

impl Image {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let decoder = png::Decoder::new(File::open(&path)?);
        let (info, mut reader) = decoder.read_info()?;
        let mut buf = vec![0; info.buffer_size()];
        reader.next_frame(&mut buf)?;

        let channels = info.color_type.samples();
        assert!(channels == 3 || channels == 4);

        Ok(Image {
            path: path.as_ref().to_str().expect("Non UTF-8 path").to_owned(),
            buf,
            wd: info.width as usize,
            ht: info.height as usize,
            channels,
        })
    }

    pub fn color(&self, u: f32, v: f32) -> Vec3 {
        let x = (u * (self.wd - 1) as f32) as usize;
        let y = (v * (self.ht - 1) as f32) as usize;
        let i = (y * self.wd + x) * self.channels;
        Vec3::new(
            self.buf[i + 0] as f32 / 255.0,
            self.buf[i + 1] as f32 / 255.0,
            self.buf[i + 2] as f32 / 255.0,
        )
    }
}

impl TryFrom<TexturePath> for Image {
    type Error = io::Error;

    fn try_from(path: TexturePath) -> Result<Self, Self::Error> {
        Image::load(path.path)
    }
}
