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
            .triangles()
            .filter(|triangle| triangle.is_front_facing(view_direction))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts;

    use glam::Vec3;

    use crate::geometry::Facet;

    use super::*;

    #[test]
    fn from_pos_z_both_facets_visible_with_neg_z_normals() {
        let mesh = flat_square_xy();
        let visible = mesh.visible_triangles(UnitVec3::Z).collect::<Vec<_>>();

        assert_eq!(visible.len(), 2);
        assert!(
            visible
                .iter()
                .all(|tri| tri.normals() == [UnitVec3::NEG_Z, UnitVec3::NEG_Z, UnitVec3::NEG_Z])
        );
    }

    #[test]
    fn from_neg_z_no_facets_visible() {
        assert_eq!(
            flat_square_xy().visible_triangles(UnitVec3::NEG_Z).count(),
            0
        );
    }

    #[test]
    fn perpendicular_view_is_grazing_neither_triangle_front() {
        let mesh = flat_square_xy();
        assert_eq!(mesh.visible_triangles(UnitVec3::X).count(), 0);
        assert_eq!(mesh.visible_triangles(UnitVec3::Y).count(), 0);
    }

    #[test]
    fn pi_rotation_y_swaps_which_direction_sees_square() {
        let mut mesh = flat_square_xy();
        mesh.pose = Mat4::from_rotation_y(consts::PI);

        assert_eq!(mesh.visible_triangles(UnitVec3::Z).count(), 0);
        assert_eq!(mesh.visible_triangles(UnitVec3::NEG_Z).count(), 2);
    }

    /// **`z = 0`**, **`[-½, ½]²`** in **XY**. Two triangles, outward **[`UnitVec3::NEG_Z`]** —
    /// visible when **into‑scene** view is **`+Z`** (same rule as **`cube`** fronts vs
    /// [`Camera::direction`](crate::Camera::direction)).
    fn flat_square_xy() -> PosedMesh {
        #[rustfmt::skip]
        let vertices = vec![
            Vec3::new(-0.5, -0.5, 0.0),
            Vec3::new( 0.5, -0.5, 0.0),
            Vec3::new( 0.5,  0.5, 0.0),
            Vec3::new(-0.5,  0.5, 0.0),
        ];
        // CCW winding viewed from **`−Z`** (outside along **`UnitVec3::NEG_Z`**).
        let facets = vec![
            Facet::with_facet_normal([0, 2, 1], UnitVec3::NEG_Z),
            Facet::with_facet_normal([0, 3, 2], UnitVec3::NEG_Z),
        ];
        let model = ModelMesh::new(vertices, facets);
        PosedMesh::new(model, Mat4::IDENTITY)
    }
}
