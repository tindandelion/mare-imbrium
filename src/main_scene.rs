mod shader;
mod texture;
mod texture_mapper;

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
    texture: Texture,
}

impl Scene {
    const MESH_LOD: usize = 5;
    const TEXTURE_PATH: &str = "assets/lroc_color_16bit_srgb_4k.tif";

    pub fn new(sun_direction: UnitVec3) -> Self {
        let texture = Texture::load_from_tif(Self::TEXTURE_PATH).expect("Failed to load texture");
        Self {
            sun_direction,
            posed_globe: PosedMesh::new(sphere(Self::MESH_LOD), Mat4::IDENTITY),
            background: Rgb::from_hex(0x111111),
            texture,
        }
    }

    pub fn set_pose_transform(&mut self, transform: Mat4) {
        self.posed_globe.pose = transform;
    }

    pub fn render(&self, framebuffer: &mut FrameBuffer, camera: &Camera) {
        framebuffer.clear(self.background);

        let shader = LunarSurfaceShader::new(self.sun_direction, &self.texture);
        camera.render(framebuffer, &self.posed_globe, &shader);
    }
}
