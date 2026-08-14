mod texture;
use crate::{
    Camera, FrameBuffer, Rgb, Shader,
    geometry::{PosedMesh, SurfacePoint, UnitVec3},
    lighting::Color,
    meshes::sphere,
};
use glam::{Mat4, Vec3};
use std::ops::{Add, Mul, Sub};

pub use texture::Texture;

pub struct Scene {
    pub sun_direction: UnitVec3,
    pub background: Rgb,
    posed_globe: PosedMesh,
}

impl Scene {
    pub fn new(sun_direction: UnitVec3, mesh_lod: usize) -> Self {
        Self {
            sun_direction,
            posed_globe: PosedMesh::new(sphere(mesh_lod), Mat4::IDENTITY),
            background: Rgb::from_hex(0x111111),
        }
    }

    pub fn set_pose_transform(&mut self, transform: Mat4) {
        self.posed_globe.pose = transform;
    }

    pub fn render(&self, framebuffer: &mut FrameBuffer, camera: &Camera) {
        framebuffer.clear(self.background);

        let shader = LunarSurfaceShader {
            toward_sun: self.sun_direction,
        };

        camera.render(framebuffer, &self.posed_globe, &shader);
    }
}

struct LunarSurfaceShader {
    toward_sun: UnitVec3,
}

impl Shader for LunarSurfaceShader {
    type VertexData = LunarSurfaceData;

    fn shade_vertex(
        &self,
        model_vertex: SurfacePoint,
        posed_vertex: SurfacePoint,
    ) -> Self::VertexData {
        LunarSurfaceData {
            model_space_pos: model_vertex.normal().into(),
            world_space_normal: posed_vertex.normal().into(),
        }
    }

    fn shade_pixel(&self, normals: Self::VertexData) -> Color {
        let illumination = self.toward_sun.dot(normals.world_space_normal).max(0.0);
        let color = Color(
            (normals.model_space_pos.x + 1.0) / 2.0,
            (normals.model_space_pos.y + 1.0) / 2.0,
            (normals.model_space_pos.z + 1.0) / 2.0,
        );
        color * illumination
    }
}

#[derive(Copy, Clone)]
struct LunarSurfaceData {
    model_space_pos: Vec3,
    world_space_normal: Vec3,
}

impl Add for LunarSurfaceData {
    type Output = LunarSurfaceData;

    fn add(self, other: Self) -> Self::Output {
        Self {
            model_space_pos: self.model_space_pos + other.model_space_pos,
            world_space_normal: self.world_space_normal + other.world_space_normal,
        }
    }
}

impl Sub for LunarSurfaceData {
    type Output = LunarSurfaceData;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            model_space_pos: self.model_space_pos - other.model_space_pos,
            world_space_normal: self.world_space_normal - other.world_space_normal,
        }
    }
}

impl Mul<f32> for LunarSurfaceData {
    type Output = LunarSurfaceData;

    fn mul(self, other: f32) -> Self::Output {
        Self {
            model_space_pos: self.model_space_pos * other,
            world_space_normal: self.world_space_normal * other,
        }
    }
}
