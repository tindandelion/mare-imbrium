//! One front-facing **triangle** in world space: three **[`SurfacePoint`]** vertices plus facet **[`UnitVec3`]** normal for culling.

use super::surface_point::SurfacePoint;
use super::unit_vec3::UnitVec3;

/// One strictly front-filled **triangle** in world space: **`vertices`** plus outward **[`UnitVec3`]** **`facet_normal`** for culling.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    pub vertices: [SurfacePoint; 3],
    pub facet_normal: UnitVec3,
}

impl Triangle {
    pub fn normals(&self) -> [UnitVec3; 3] {
        self.vertices.map(|v| v.normal())
    }

    pub fn is_front_facing(&self, view_direction: UnitVec3) -> bool {
        view_direction.dot(self.facet_normal) < 0.0
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;

    #[test]
    fn is_front_facing_true_for_neg_z_normal_when_view_is_pos_z() {
        let facet_normal = UnitVec3::NEG_Z;
        let triangle = Triangle {
            vertices: vertices(facet_normal),
            facet_normal,
        };
        assert!(triangle.is_front_facing(UnitVec3::Z));
    }

    #[test]
    fn is_front_facing_false_for_pos_z_normal_when_view_is_pos_z() {
        let facet_normal = UnitVec3::Z;
        let triangle = Triangle {
            vertices: vertices(facet_normal),
            facet_normal,
        };

        assert!(!triangle.is_front_facing(UnitVec3::Z));
    }

    #[test]
    fn is_front_facing_false_when_grazing() {
        let facet_normal = UnitVec3::X;
        let triangle = Triangle {
            vertices: vertices(facet_normal),
            facet_normal,
        };

        assert!(!triangle.is_front_facing(UnitVec3::Z));
    }

    fn vertices(normal: UnitVec3) -> [SurfacePoint; 3] {
        [
            SurfacePoint::new(Vec3::new(0.0, 0.0, 0.0), normal),
            SurfacePoint::new(Vec3::new(0.0, 0.0, 1.0), normal),
            SurfacePoint::new(Vec3::new(0.0, 1.0, 0.0), normal),
        ]
    }
}
