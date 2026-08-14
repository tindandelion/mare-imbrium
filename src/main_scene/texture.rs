use std::{fs::File, io};

use glam::Vec3;
use png::Transformations;

use crate::{Rgb, lighting::Color};

use super::texture_mapper::TextureCoordMapper;

pub struct Texture {
    width: usize,
    coord_mapper: TextureCoordMapper,
    pixels: Vec<Color>,
}

impl Default for Texture {
    fn default() -> Self {
        Self::new(1, 1, vec![Color(1.0, 0.0, 0.0)])
    }
}

impl Texture {
    fn new(width: usize, height: usize, pixels: Vec<Color>) -> Self {
        let coord_mapper = TextureCoordMapper::new(width, height);
        Self {
            width,
            pixels,
            coord_mapper,
        }
    }

    pub fn solid_color(color: Color, width: usize, height: usize) -> Self {
        Self::new(width, height, vec![color; width * height])
    }

    pub fn load_from_png(path: &str) -> io::Result<Self> {
        let mut decoder = png::Decoder::new(File::open(path)?);
        decoder.set_transformations(Transformations::normalize_to_color8());

        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let pixels: Vec<Color> = buf[..info.buffer_size()]
            .chunks_exact(3)
            .map(|chunk| Rgb(chunk[0], chunk[1], chunk[2]).into())
            .collect();

        Ok(Self::new(info.width as usize, info.height as usize, pixels))
    }

    pub fn get_pixel(&self, normal: Vec3) -> Color {
        debug_assert!(normal.is_normalized());

        let (u, v) = self.coord_mapper.to_uv_coords(normal);
        self.pixels[v * self.width + u]
    }
}
