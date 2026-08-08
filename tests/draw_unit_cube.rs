//! Integration: **`Shape::render`** on the default unit cube vs a hand-built golden framebuffer.
//!
//! The golden fills the projected **−Z** cap with a local **`fill_rect`** helper (duplicated facet normals → uniform
//! intensity, so the shaded square matches a flat fill). With **`Camera::direction` = +Z**, the strictly
//! front-facing hull facet is the **−Z** cap (outward normal **`NEG_Z`**). **`Light::directional`** toward **`NEG_Z`**
//! with a high-ambient matte yields uniform intensity, so
//! the material **`emissive`** is unchanged when **`diffuse`** is black. On this **`FB_WIDTH`×`FB_HEIGHT`** canvas,
//! **`scale = (min(w,h) − 1) / 2`** is an integer, so unit-cube **`±0.5`** corners land exactly on
//! **`FILLED_MIN…FILLED_LAST`** (**no** intermediate **`f32::round`**).

use glam::{Mat4, UVec2, Vec3};
use mare_imbrium::{
    Camera, FrameBuffer, Light, Material, Rgb, Shape, framebuffer::FbPixel, meshes::cube,
    shaders::PhongShader,
};

const FB_WIDTH: u32 = 101;
const FB_HEIGHT: u32 = 101;
const CAMERA_POS: Vec3 = Vec3::new(0.0, 0.0, -1.0);

#[test]
fn draw_single_unit_cube_produces_rectangle() {
    let mut fb = FrameBuffer::new(FB_WIDTH, FB_HEIGHT);
    let camera = Camera::for_viewport(FB_WIDTH, FB_HEIGHT).move_to(CAMERA_POS);

    let (shape, material) = positioned_cube(0.0, Rgb::BLUE);
    render_shape(&shape, &mut fb, &camera, &material);

    let expected = framebuffer_with_rectangle(UVec2::new(25, 25), UVec2::new(75, 75), Rgb::BLUE);
    assert_eq!(fb.as_ref(), expected.as_ref());
}

#[test]
fn draw_occluded_cubes_hides_far_cube() {
    let mut fb = FrameBuffer::new(FB_WIDTH, FB_HEIGHT);
    let camera = Camera::for_viewport(FB_WIDTH, FB_HEIGHT).move_to(CAMERA_POS);
    let (near_shape, near_material) = positioned_cube(0.0, Rgb::BLUE);
    let (far_shape, far_material) = positioned_cube(2.0, Rgb::RED);

    render_shape(&near_shape, &mut fb, &camera, &near_material);
    render_shape(&far_shape, &mut fb, &camera, &far_material);

    let expected = framebuffer_with_rectangle(UVec2::new(25, 25), UVec2::new(75, 75), Rgb::BLUE);
    assert_eq!(fb.as_ref(), expected.as_ref());
}

fn positioned_cube(z_position: f32, color: Rgb) -> (Shape, Material) {
    let material = Material::from_rgb(color, Rgb::BLACK, Rgb::BLACK, None);
    (
        Shape::new(cube().transform(Mat4::from_translation(Vec3::new(0.0, 0.0, z_position)))),
        material,
    )
}

fn framebuffer_with_rectangle(top_left: UVec2, bottom_right: UVec2, color: Rgb) -> FrameBuffer {
    let mut fb = FrameBuffer::new(FB_WIDTH, FB_HEIGHT);
    for y in top_left.y..=bottom_right.y {
        for x in top_left.x..=bottom_right.x {
            fb.write_pixel(FbPixel::new(x, y, 0.0), color);
        }
    }
    fb
}

fn render_shape(shape: &Shape, fb: &mut FrameBuffer, camera: &Camera, material: &Material) {
    let light = Light::directional(-camera.direction(), 1.0);
    let shader = PhongShader {
        material,
        lights: &[light],
        toward_eye: -camera.direction(),
    };

    camera.render(fb, shape, &shader);
}
