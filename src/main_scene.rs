use glam::{Mat4, Vec3};

use crate::{
    Camera, FrameBuffer, Rgb, Shader,
    geometry::{PosedMesh, SurfacePoint, UnitVec3},
    lighting::Color,
    meshes::sphere,
};

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

impl LunarSurfaceShader {
    const COLOR: Color = Color(0.5, 0.5, 0.5);
}

impl Shader for LunarSurfaceShader {
    type VertexData = Vec3;

    fn shade_vertex(&self, vertex: SurfacePoint) -> Self::VertexData {
        vertex.normal().into()
    }

    fn shade_pixel(&self, data: Self::VertexData) -> Color {
        let illumination = self.toward_sun.dot(data).max(0.0);
        Self::COLOR * illumination
    }
}
