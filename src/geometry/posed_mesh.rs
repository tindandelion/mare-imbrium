use glam::Mat4;

use super::{ModelMesh, Triangle, UnitVec3};

pub struct PosedMesh {
    pub model: ModelMesh,
    pub pose: Mat4,
}

impl PosedMesh {
    pub fn new(model: ModelMesh, pose: Mat4) -> Self {
        Self { model, pose }
    }

    pub fn visible_triangles(
        &self,
        view_direction: UnitVec3,
    ) -> impl Iterator<Item = Triangle> + '_ {
        let posed_mesh = self.model.transform(self.pose);
        posed_mesh
            .visible_triangles(view_direction)
            .collect::<Vec<_>>()
            .into_iter()
    }
}
