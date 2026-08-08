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
use crate::geometry::{PosedMesh, SurfacePoint};
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

    fn shade_vertex(
        &self,
        model_vertex: SurfacePoint,
        posed_vertex: SurfacePoint,
    ) -> Self::VertexData;
    fn shade_pixel(&self, data: Self::VertexData) -> Color;
}

impl Camera {
    pub fn render<S: Shader>(&self, fb: &mut FrameBuffer, mesh: &PosedMesh, shader: &S) {
        for (model_tri, posed_tri) in mesh.visible_triangles_2(self.direction()) {
            let corners = array::from_fn(|i| {
                let posed_vertex = posed_tri.vertices[i];
                let model_vertex = model_tri.vertices[i];
                ShadedCorner {
                    pixel: self.transform(posed_vertex.position()),
                    value: shader.shade_vertex(model_vertex, posed_vertex),
                }
            });
            ShadedTriangle::new(corners)
                .draw(fb, |surface_pt| Rgb::from(shader.shade_pixel(surface_pt)));
        }
    }
}
