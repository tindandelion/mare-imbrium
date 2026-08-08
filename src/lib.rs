//! **Thorus Forge** — software rasterizer building blocks: RGB framebuffer, orthographic screen mapping, lossless WebP encode.
//!
//! Shared raster canvas size and **`animated-scene`** timing constants live here (see **`doc/planning/project-spec.md`**).

use std::array;

pub mod framebuffer;
pub mod geometry;
pub mod lighting;
pub mod main_scene;
pub mod meshes;
pub mod ortho_camera;
pub mod shaders;
pub mod webp_encoder;

pub use framebuffer::{FrameBuffer, Rgb};
pub use lighting::{Light, Material};
pub use ortho_camera::Camera;
pub use webp_encoder::WebpEncoder;

use crate::framebuffer::{Interpolatable, ShadedCorner, ShadedTriangle};
use crate::geometry::{Mesh, SurfacePoint};
use crate::lighting::Color;

impl Material {
    pub fn from_rgb(emissive: Rgb, diffuse: Rgb, specular: Rgb, shininess: Option<i32>) -> Self {
        Self::new(
            Color::from(emissive),
            Color::from(diffuse),
            Color::from(specular),
            shininess,
        )
    }
}

pub trait Shader {
    type VertexData: Interpolatable;

    fn shade_vertex(&self, vertex: SurfacePoint) -> Self::VertexData;
    fn shade_pixel(&self, data: Self::VertexData) -> Color;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Shape {
    pub model: Mesh,
}

impl Shape {
    pub fn new(model: Mesh) -> Self {
        Self { model }
    }
}

impl Camera {
    pub fn render<S: Shader>(&self, fb: &mut FrameBuffer, shape: &Shape, shader: &S) {
        for triangle in shape.model.visible_triangles(self.direction()) {
            let corners = array::from_fn(|i| {
                let vertex = triangle.vertices[i];
                ShadedCorner {
                    pixel: self.transform(vertex.position()),
                    value: shader.shade_vertex(vertex),
                }
            });
            ShadedTriangle::new(corners)
                .draw(fb, |surface_pt| Rgb::from(shader.shade_pixel(surface_pt)));
        }
    }
}
