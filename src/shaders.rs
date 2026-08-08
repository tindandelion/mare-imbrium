use crate::{
    Light, Material, Shader,
    geometry::{SurfacePoint, UnitVec3},
    lighting::Color,
};

pub struct PhongShader<'a> {
    pub material: &'a Material,
    pub lights: &'a [Light],
    pub toward_eye: UnitVec3,
}

impl<'a> Shader for PhongShader<'a> {
    type VertexData = SurfacePoint;

    fn shade_vertex(
        &self,
        _model_vertex: SurfacePoint,
        posed_vertex: SurfacePoint,
    ) -> Self::VertexData {
        posed_vertex
    }

    fn shade_pixel(&self, surface_point: SurfacePoint) -> Color {
        self.material
            .shade(self.lights, surface_point, self.toward_eye)
    }
}

pub struct GouraudShader<'a> {
    pub material: &'a Material,
    pub lights: &'a [Light],
    pub toward_eye: UnitVec3,
}

impl<'a> Shader for GouraudShader<'a> {
    type VertexData = Color;

    fn shade_vertex(
        &self,
        _model_vertex: SurfacePoint,
        posed_vertex: SurfacePoint,
    ) -> Self::VertexData {
        self.material
            .shade(self.lights, posed_vertex, self.toward_eye)
    }

    fn shade_pixel(&self, color: Color) -> Color {
        color
    }
}
