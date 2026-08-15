mod frame_writer;

use crate::frame_writer::PngFrameWriter;
use glam::{Mat4, Vec3};
use mare_imbrium::{Camera, FrameBuffer, WebpEncoder, main_scene::Scene};
use std::f32::consts::TAU;
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const CAMERA_POS: Vec3 = Vec3::new(0.0, 0.0, -1.0);
const SUN_DIRECTION: Vec3 = Vec3::new(1.0, 1.0, -1.2);

const GLOBE_SCALE: Vec3 = Vec3::splat(0.9);

const SCENE_WIDTH: u32 = 800;
const SCENE_HEIGHT: u32 = 800;
pub const ANIMATED_SCENE_FRAME_COUNT: u32 = 360;
pub const ANIMATED_SCENE_FRAME_SPACING_MS: i32 = 20;

const BASE_OUT_DIR: &str = concat!("target/", env!("CARGO_BIN_NAME"));

fn model_transform(angle: f32) -> Mat4 {
    let rotation = Mat4::from_rotation_y(angle);
    Mat4::from_scale(GLOBE_SCALE) * rotation
}

fn main() {
    let (tx, rx) = mpsc::channel();

    let render_thread = thread::spawn(move || render_frames(tx));

    write_frames(rx);
    println!("*** Finished writing frames");

    let (frame_production_fps, frame_production_elapsed) = render_thread.join().unwrap();
    println!(
        "Frame production: {:.2} fps ({ANIMATED_SCENE_FRAME_COUNT} frames in {:?})",
        frame_production_fps, frame_production_elapsed,
    );
}

pub fn render_frames(tx: mpsc::Sender<FrameBuffer>) -> (f64, Duration) {
    let camera = Camera::for_viewport(SCENE_WIDTH, SCENE_HEIGHT).move_to(CAMERA_POS);
    let mut scene = Scene::new(SUN_DIRECTION.into());

    let frame_production_start = std::time::Instant::now();
    let lap_frames = ANIMATED_SCENE_FRAME_COUNT.max(1) as f32;

    for frame_index in 0..ANIMATED_SCENE_FRAME_COUNT {
        println!("Rendering frame {frame_index} of {ANIMATED_SCENE_FRAME_COUNT}");
        let t = frame_index as f32 / lap_frames * TAU;
        scene.set_pose_transform(model_transform(t));

        let mut framebuffer = FrameBuffer::new(SCENE_WIDTH, SCENE_HEIGHT);
        scene.render(&mut framebuffer, &camera);

        tx.send(framebuffer).expect("Failed to send framebuffer");
    }

    println!("*** Finished rendering frames");
    let frame_production_elapsed = frame_production_start.elapsed();
    let frame_production_secs = frame_production_elapsed.as_secs_f64().max(1e-12);
    let frame_production_fps = ANIMATED_SCENE_FRAME_COUNT as f64 / frame_production_secs;

    (frame_production_fps, frame_production_elapsed)
}

pub fn write_frames(rx: mpsc::Receiver<FrameBuffer>) {
    let png_writer = PngFrameWriter::new(
        Path::new(BASE_OUT_DIR).join("frames"),
        SCENE_WIDTH,
        SCENE_HEIGHT,
    );
    let mut webp_encoder =
        WebpEncoder::with_frame_spacing(SCENE_WIDTH, SCENE_HEIGHT, ANIMATED_SCENE_FRAME_SPACING_MS)
            .expect("Failed to create webp encoder");

    png_writer.clear().expect("Failed to clear png writer");
    for (index, framebuffer) in rx.iter().enumerate() {
        png_writer
            .write_frame(index as u32, &framebuffer)
            .expect("Failed to write png frame");
        webp_encoder
            .add_frame(&framebuffer)
            .expect("Failed to add frame to webp encoder");
        println!("Written frame {index} of {ANIMATED_SCENE_FRAME_COUNT}");
    }

    webp_encoder
        .write(Path::new(BASE_OUT_DIR).join("lunar-globe.webp"))
        .expect("Failed to write webp");
}
