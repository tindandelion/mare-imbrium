use std::{f32::consts, fs::File, io};

use png::Transformations;

use crate::{Rgb, lighting::Color};

pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            width: 1,
            height: 1,
            pixels: vec![Color(1.0, 0.0, 0.0)],
        }
    }
}

impl Texture {
    pub fn solid_color(color: Color, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![color; width * height],
        }
    }

    pub fn load_from_png(path: &str) -> io::Result<Self> {
        let mut decoder = png::Decoder::new(File::open(path)?);
        decoder.set_transformations(Transformations::normalize_to_color8());

        let mut reader = decoder.read_info()?;
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf)?;

        let pixels: Vec<Color> = (&buf[..info.buffer_size()])
            .chunks_exact(3)
            .map(|chunk| Rgb(chunk[0], chunk[1], chunk[2]).into())
            .collect();

        Ok(Self {
            width: info.width as usize,
            height: info.height as usize,
            pixels,
        })
    }

    pub fn get_pixel_polar(&self, azimuth: f32, polar: f32) -> Color {
        let (u, v) = self.convert_to_uv(azimuth, polar);
        return self.pixels[v * self.width + u];
    }

    fn convert_to_uv(&self, azimuth: f32, polar: f32) -> (usize, usize) {
        debug_assert!(
            azimuth >= 0.0 && azimuth < consts::TAU,
            "azimuth: {}",
            azimuth
        );
        debug_assert!(polar >= 0.0 && polar < consts::PI, "polar: {}", polar);

        let u = (self.width as f32 * azimuth / consts::TAU) as usize;
        let v = (self.height as f32 * polar / consts::PI) as usize;
        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn load_texture_from_png() {
        let texture = Texture::load_from_png("assets/texture.png").unwrap();

        let upper_left_pixel = texture.get_pixel_polar(0.0, 0.0);
        let center_pixel = texture.get_pixel_polar(consts::PI, consts::FRAC_PI_2);
        let bottom_right_pixel = texture.get_pixel_polar(consts::TAU - 0.0001, consts::PI - 0.0001);

        assert_eq!(Color(0.0, 1.0, 1.0), upper_left_pixel);
        assert_eq!(Color(1.0, 0.0, 0.0), center_pixel);
        assert_eq!(Color(0.0, 1.0, 0.0), bottom_right_pixel);
    }

    #[test]
    fn convert_to_uv_along_azimuth_line() {
        let texture = Texture::solid_color(Color(0.0, 0.0, 0.0), 2, 2);

        assert_eq!((0, 0), texture.convert_to_uv(0.0, 0.0));
        assert_eq!((0, 0), texture.convert_to_uv(consts::FRAC_PI_2, 0.0));
        assert_eq!((0, 0), texture.convert_to_uv(consts::PI - 0.0001, 0.0));
        assert_eq!((1, 0), texture.convert_to_uv(consts::PI, 0.0));

        assert_eq!(
            (1, 0),
            texture.convert_to_uv(consts::PI + consts::FRAC_PI_2, 0.0)
        );

        assert_eq!((1, 0), texture.convert_to_uv(consts::TAU - 0.0001, 0.0));
    }

    #[test]
    fn convert_to_uv_along_polar_line() {
        let texture = Texture::solid_color(Color(0.0, 0.0, 0.0), 2, 2);

        assert_eq!((0, 0), texture.convert_to_uv(0.0, 0.0));
        assert_eq!(
            (0, 0),
            texture.convert_to_uv(0.0, consts::FRAC_PI_2 - 0.0001)
        );

        assert_eq!((0, 1), texture.convert_to_uv(0.0, consts::FRAC_PI_2));
        assert_eq!(
            (0, 1),
            texture.convert_to_uv(0.0, consts::FRAC_PI_2 + 0.0001)
        );
        assert_eq!((0, 1), texture.convert_to_uv(0.0, consts::PI - 0.0001));
    }
}
