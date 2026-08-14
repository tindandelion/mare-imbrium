use std::{f32::consts, fs::File, io};

use png::Transformations;

use crate::{Rgb, lighting::Color};

pub struct Texture {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Texture {
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

    fn get_pixel_polar(&self, azimuth: f32, polar: f32) -> Color {
        let u = ((self.width - 1) as f32 * azimuth / consts::TAU) as usize;
        let v = ((self.height - 1) as f32 * polar / consts::PI) as usize;
        return self.pixels[v * self.width + u];
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
        let bottom_right_pixel = texture.get_pixel_polar(consts::TAU, consts::PI);

        assert_eq!(Color(0.0, 1.0, 1.0), upper_left_pixel);
        assert_eq!(Color(1.0, 0.0, 0.0), center_pixel);
        assert_eq!(Color(0.0, 1.0, 0.0), bottom_right_pixel);
    }
}
