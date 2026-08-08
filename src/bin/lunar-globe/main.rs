mod frame_writer;

use crate::frame_writer::PngFrameWriter;
use glam::{Mat4, Vec3};
use mare_imbrium::{Camera, FrameBuffer, WebpEncoder, main_scene::Scene};
use std::f32::consts::TAU;
use std::path::Path;

const CAMERA_POS: Vec3 = Vec3::new(0.0, 0.0, -1.0);
const SUN_DIRECTION: Vec3 = Vec3::new(1.0, 1.0, -0.5);

const GLOBE_TESSELATION: usize = 5;
const GLOBE_SCALE: f32 = 0.9;

const SCENE_WIDTH: u32 = 800;
const SCENE_HEIGHT: u32 = 800;
pub const ANIMATED_SCENE_FRAME_COUNT: u32 = 360;
pub const ANIMATED_SCENE_FRAME_SPACING_MS: i32 = 20;

const BASE_OUT_DIR: &str = concat!("target/", env!("CARGO_BIN_NAME"));

fn model_matrix_tumble(t: f32) -> Mat4 {
    let rotation = Mat4::from_rotation_y(t);
    Mat4::from_scale(Vec3::splat(GLOBE_SCALE)) * rotation
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png_writer = PngFrameWriter::new(
        Path::new(BASE_OUT_DIR).join("frames"),
        SCENE_WIDTH,
        SCENE_HEIGHT,
    );
    let mut webp_encoder = WebpEncoder::with_frame_spacing(
        SCENE_WIDTH,
        SCENE_HEIGHT,
        ANIMATED_SCENE_FRAME_SPACING_MS,
    )?;

    png_writer.clear()?;
    let mut framebuffer = FrameBuffer::new(SCENE_WIDTH, SCENE_HEIGHT);
    let camera = Camera::for_viewport(SCENE_WIDTH, SCENE_HEIGHT).move_to(CAMERA_POS);

    let mut scene = Scene::new(SUN_DIRECTION.into(), GLOBE_TESSELATION);

    let frame_production_start = std::time::Instant::now();
    let lap_frames = ANIMATED_SCENE_FRAME_COUNT.max(1) as f32;

    for frame_index in 0..ANIMATED_SCENE_FRAME_COUNT {
        let t = frame_index as f32 / lap_frames * TAU;
        scene.set_pose_transform(model_matrix_tumble(t));
        scene.render(&mut framebuffer, &camera);

        png_writer.write_frame(frame_index, &framebuffer)?;
        webp_encoder.add_frame(&framebuffer)?;
    }
    let frame_production_elapsed = frame_production_start.elapsed();
    let frame_production_secs = frame_production_elapsed.as_secs_f64().max(1e-12);
    let frame_production_fps = ANIMATED_SCENE_FRAME_COUNT as f64 / frame_production_secs;

    webp_encoder.write(Path::new(BASE_OUT_DIR).join("lunar-globe.webp"))?;

    println!(
        "Frame production: {:.2} fps ({ANIMATED_SCENE_FRAME_COUNT} frames in {:?})",
        frame_production_fps, frame_production_elapsed,
    );

    Ok(())
}
