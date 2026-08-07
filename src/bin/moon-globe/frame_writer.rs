use std::{
    fs::{self, File},
    io::{self, BufWriter},
    path::PathBuf,
};

use png::{BitDepth, ColorType, Encoder};

pub struct PngFrameWriter {
    out_dir: PathBuf,
    width: u32,
    height: u32,
}

impl PngFrameWriter {
    pub fn new(out_dir: impl Into<PathBuf>, width: u32, height: u32) -> Self {
        Self {
            out_dir: out_dir.into(),
            width,
            height,
        }
    }

    pub fn clear(&self) -> io::Result<()> {
        if self.out_dir.exists() {
            fs::remove_dir_all(&self.out_dir)?;
        }
        fs::create_dir_all(&self.out_dir)?;
        Ok(())
    }

    pub fn write_frame(&self, frame_index: u32, rgb: impl AsRef<[u8]>) -> io::Result<()> {
        let writer = self.create_file_writer(frame_index)?;

        let mut encoder = Encoder::new(writer, self.width, self.height);
        encoder.set_color(ColorType::Rgb);
        encoder.set_depth(BitDepth::Eight);

        let mut writer = encoder.write_header()?;
        writer.write_image_data(rgb.as_ref())?;
        Ok(())
    }

    fn create_file_writer(&self, frame_index: u32) -> io::Result<BufWriter<File>> {
        let path = self.out_dir.join(format!("{frame_index:06}.png"));
        Ok(BufWriter::new(File::create(path)?))
    }
}
