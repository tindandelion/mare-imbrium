use std::{fs::File, io};

use glam::Vec3;
use png::Transformations;

use crate::{Rgb, framebuffer::srgb_normalized_to_linear, lighting::Color};

use super::texture_mapper::TextureCoordMapper;

pub struct ColorMap {
    coord_mapper: TextureCoordMapper,
    pixels: Vec<Color>,
}

impl ColorMap {
    fn new(width: usize, height: usize, pixels: Vec<Color>) -> Self {
        let coord_mapper = TextureCoordMapper::new(width, height);
        Self {
            pixels,
            coord_mapper,
        }
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

    pub fn load_from_tif(path: &str) -> io::Result<Self> {
        let mut decoder = tiff::decoder::Decoder::new(File::open(path)?).map_err(tiff_to_io)?;

        match decoder.colortype().map_err(tiff_to_io)? {
            tiff::ColorType::RGB(16) => {}
            color_type => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("expected 16-bit RGB TIFF, got {color_type:?}"),
                ));
            }
        }

        let (width, height) = decoder.dimensions().map_err(tiff_to_io)?;
        let tiff::decoder::DecodingResult::U16(samples) =
            decoder.read_image().map_err(tiff_to_io)?
        else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "expected 16-bit TIFF samples",
            ));
        };

        let pixels = samples
            .chunks_exact(3)
            .map(|chunk| {
                Color(
                    srgb16_channel_to_linear(chunk[0]),
                    srgb16_channel_to_linear(chunk[1]),
                    srgb16_channel_to_linear(chunk[2]),
                )
            })
            .collect();

        Ok(Self::new(width as usize, height as usize, pixels))
    }

    pub fn get_pixel(&self, normal: Vec3) -> Color {
        debug_assert!(normal.is_normalized());

        let (u, v) = self.coord_mapper.to_uv_coords(normal);
        self.pixels[v * self.coord_mapper.width + u]
    }
}

fn tiff_to_io(err: tiff::TiffError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, err)
}

fn srgb16_channel_to_linear(channel: u16) -> f32 {
    srgb_normalized_to_linear(channel as f32 / 65535.0)
}
