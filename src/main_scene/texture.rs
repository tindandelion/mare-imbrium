use std::{fs::File, io};

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

    fn get_pixel(&self, x: usize, y: usize) -> Color {
        let index = y * self.width + x;
        return self.pixels[index];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_texture_from_png() {
        let texture = Texture::load_from_png("assets/texture.png").unwrap();

        let upper_left_pixel = texture.get_pixel(0, 0);
        let center_pixel = texture.get_pixel(63, 31);

        assert_eq!(Color(0.0, 1.0, 1.0), upper_left_pixel);
        assert_eq!(Color(1.0, 0.0, 0.0), center_pixel);
    }
}
