use crate::{
    Shader,
    geometry::{SurfacePoint, UnitVec3},
    lighting::Color,
};
use glam::Vec3;
use std::ops::{Add, Mul, Sub};

use super::Texture;

pub struct LunarSurfaceShader {
    toward_sun: UnitVec3,
    texture: Texture,
}

impl LunarSurfaceShader {
    pub fn new(toward_sun: UnitVec3, texture: Texture) -> Self {
        Self {
            toward_sun,
            texture,
        }
    }
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
        let illumination = self
            .toward_sun
            .dot(normals.world_space_normal.normalize())
            .max(0.0);

        let color = self.texture.get_pixel(normals.model_space_pos.normalize());
        color * illumination
    }
}

#[derive(Copy, Clone)]
pub struct LunarSurfaceData {
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
