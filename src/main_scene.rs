use glam::{Mat4, Vec3};

use crate::{
    Camera, FrameBuffer, Rgb, Shader, Shape,
    geometry::{ModelMesh, SurfacePoint, UnitVec3},
    lighting::Color,
    meshes::sphere,
};

pub struct Scene {
    pub sun_direction: UnitVec3,
    pub globe_mesh: ModelMesh,
    pub transform: Mat4,
    pub background: Rgb,
}

impl Scene {
    pub fn new(sun_direction: UnitVec3, mesh_lod: usize) -> Self {
        Self {
            sun_direction,
            transform: Mat4::IDENTITY,
            globe_mesh: sphere(mesh_lod),
            background: Rgb::from_hex(0x111111),
        }
    }

    pub fn render(&self, framebuffer: &mut FrameBuffer, camera: &Camera) {
        framebuffer.clear(self.background);

        let shader = LunarSurfaceShader {
            toward_sun: self.sun_direction,
        };

        let shape = Shape::new(self.globe_mesh.transform(self.transform));
        camera.render(framebuffer, &shape, &shader);
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
