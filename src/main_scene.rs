mod shader;
mod texture;

use crate::{
    Camera, FrameBuffer, Rgb,
    geometry::{PosedMesh, UnitVec3},
    meshes::sphere,
};
use glam::Mat4;

use shader::LunarSurfaceShader;
pub use texture::Texture;

pub struct Scene {
    pub sun_direction: UnitVec3,
    pub background: Rgb,
    posed_globe: PosedMesh,
}

impl Scene {
    const MESH_LOD: usize = 5;

    pub fn new(sun_direction: UnitVec3) -> Self {
        Self {
            sun_direction,
            posed_globe: PosedMesh::new(sphere(Self::MESH_LOD), Mat4::IDENTITY),
            background: Rgb::from_hex(0x111111),
        }
    }

    pub fn set_pose_transform(&mut self, transform: Mat4) {
        self.posed_globe.pose = transform;
    }

    pub fn render(&self, framebuffer: &mut FrameBuffer, camera: &Camera) {
        framebuffer.clear(self.background);

        let shader = LunarSurfaceShader::new(self.sun_direction);
        camera.render(framebuffer, &self.posed_globe, &shader);
    }
}
