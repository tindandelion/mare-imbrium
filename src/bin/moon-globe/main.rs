mod frame_writer;

use std::f32::consts::TAU;
use std::path::Path;

use glam::{Mat4, Vec3};
use mare_imbrium::meshes::sphere;

use mare_imbrium::shaders::PhongShader;
use mare_imbrium::{
    ANIMATED_SCENE_FRAME_COUNT, ANIMATED_SCENE_FRAME_SPACING_MS, Camera, FrameBuffer, Light,
    Material, Rgb, SCENE_BACKGROUND, SCENE_HEIGHT, SCENE_WIDTH, Shape, WebpEncoder,
};

use crate::frame_writer::PngFrameWriter;

const CAMERA_POS: Vec3 = Vec3::new(0.0, 0.0, -1.0);
const SUN_DIRECTION: Vec3 = Vec3::new(0.0, 2.0, -0.5);

const GLOBE_TESSELATION: usize = 5;
const GLOBE_SCALE: f32 = 0.9;

const WEBP_OUT_PATH: &str = "target/scene.webp";
const PNG_OUT_DIR: &str = "target/animated-scene";

/// Uniform scale plus world-fixed **`R_z R_y R_x`** at angle **`t`** (radians).
fn model_matrix_tumble(t: f32) -> Mat4 {
    let rotation = Mat4::from_rotation_z(t) * Mat4::from_rotation_y(t) * Mat4::from_rotation_x(t);
    Mat4::from_scale(Vec3::splat(GLOBE_SCALE)) * rotation
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png_writer = PngFrameWriter::new(PNG_OUT_DIR, SCENE_WIDTH, SCENE_HEIGHT);
    let mut webp_encoder = WebpEncoder::with_frame_spacing(
        SCENE_WIDTH,
        SCENE_HEIGHT,
        ANIMATED_SCENE_FRAME_SPACING_MS,
    )?;

    png_writer.clear()?;
    let mut framebuffer = FrameBuffer::new(SCENE_WIDTH, SCENE_HEIGHT);
    let camera = Camera::for_viewport(SCENE_WIDTH, SCENE_HEIGHT).move_to(CAMERA_POS);
    let sun = [Light::directional(SUN_DIRECTION.into(), 1.0)];
    let base_mesh = sphere(GLOBE_TESSELATION);
    let material = Material::from_rgb(Rgb::BLACK, Rgb::from_hex(0xBEBEB8), Rgb::BLACK, None);

    let frame_production_start = std::time::Instant::now();
    let lap_frames = ANIMATED_SCENE_FRAME_COUNT.max(1) as f32;

    let shader = PhongShader {
        material: &material,
        lights: &sun,
        toward_eye: -camera.direction(),
    };

    for frame_index in 0..ANIMATED_SCENE_FRAME_COUNT {
        framebuffer.clear(SCENE_BACKGROUND);

        let t = frame_index as f32 / lap_frames * TAU;
        let shape = Shape::new(base_mesh.transform(model_matrix_tumble(t)));

        shape.render(&mut framebuffer, &camera, &shader);
        png_writer.write_frame(frame_index, &framebuffer)?;
        webp_encoder.add_frame(&framebuffer)?;
    }
    let frame_production_elapsed = frame_production_start.elapsed();
    let frame_production_secs = frame_production_elapsed.as_secs_f64().max(1e-12);
    let frame_production_fps = ANIMATED_SCENE_FRAME_COUNT as f64 / frame_production_secs;

    webp_encoder.write(Path::new(WEBP_OUT_PATH))?;

    println!(
        "Frame production: {:.2} fps ({ANIMATED_SCENE_FRAME_COUNT} frames in {:?})",
        frame_production_fps, frame_production_elapsed,
    );

    Ok(())
}
