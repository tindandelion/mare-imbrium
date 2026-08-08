//! Indexed triangle mesh in **model space**, backed by **`Vec<glam::Vec3>`** + **`Vec<Facet>`**.
//!
//! Construction takes arbitrary **vertex positions** + **facet list** (**CCW**, outward facet normal); **[`ModelMesh::transform`](ModelMesh::transform)** poses like procedural **[`cube`](crate::meshes::cube)** or **[`dodecahedron`](crate::meshes::dodecahedron)**.

use glam::{Mat4, Vec3};

use crate::geometry::SurfacePoint;

use super::facet::{Facet, NormalTransform};
use super::triangle::Triangle;

/// Model-space mesh: **`Facet::verts`** index into vertex positions from **[`vertices`](ModelMesh::vertices)**.
///
/// Procedural builders: **[`cube`](crate::meshes::cube)**, **[`dodecahedron`](crate::meshes::dodecahedron)**.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelMesh {
    vertices: Vec<Vec3>,
    facets: Vec<Facet>,
}

impl ModelMesh {
    pub fn new(vertices: Vec<Vec3>, facets: Vec<Facet>) -> Self {
        Self { vertices, facets }
    }

    #[inline]
    pub fn vertices(&self) -> &[Vec3] {
        &self.vertices
    }

    #[inline]
    pub fn facets(&self) -> &[Facet] {
        &self.facets
    }

    /// Applies **`Mat4::transform_point3`** per vertex and re-poses stored normals per facet
    /// (inverse-transpose of **`m`**'s upper-left **3×3**, computed once for all facets).
    pub fn transform(&self, m: Mat4) -> ModelMesh {
        let normal_transform = NormalTransform::from_model(m);
        ModelMesh {
            vertices: self
                .vertices
                .iter()
                .copied()
                .map(|v| m.transform_point3(v))
                .collect(),
            facets: self
                .facets
                .iter()
                .map(|f| f.transform(normal_transform))
                .collect(),
        }
    }

    pub fn triangles(&self) -> impl Iterator<Item = Triangle> + '_ {
        self.facets.iter().map(|facet| {
            let corners = facet.resolve_vertices(&self.vertices);
            let normals = facet.vertex_normals();
            let vertices = std::array::from_fn(|i| SurfacePoint::new(corners[i], normals[i]));

            Triangle {
                vertices,
                facet_normal: facet.facet_normal(),
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_relative_eq;

    use crate::geometry::UnitVec3;

    use super::*;
    use glam::Mat4;

    #[test]
    fn transform_applies_to_facet_normal() {
        let vertices = [
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 2.0),
            Vec3::new(0.0, 2.0, 0.0),
        ];
        let facet_normal = UnitVec3::from_points_ccw(&vertices);
        let mesh = ModelMesh::new(
            vertices.to_vec(),
            vec![Facet::with_facet_normal([0, 1, 2], facet_normal)],
        );

        let transformed_mesh = mesh.transform(Mat4::from_scale(Vec3::new(1.0, 0.5, 1.0)));
        let transformed_facet = transformed_mesh.facets()[0];
        let transformed_vertices = transformed_facet.resolve_vertices(transformed_mesh.vertices());

        let expected_transformed_normal = UnitVec3::from_points_ccw(&transformed_vertices);
        let transformed_facet_normal = transformed_mesh.facets()[0].facet_normal();

        assert_relative_eq!(expected_transformed_normal, transformed_facet_normal);
        for (actual, expected) in transformed_facet
            .vertex_normals()
            .iter()
            .zip([expected_transformed_normal; 3])
        {
            assert_relative_eq!(*actual, expected);
        }
    }
}
