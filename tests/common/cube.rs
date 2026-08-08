//! Axis-aligned **unit cube** (edge length **1**, **`[-½, ½]³`**) built as **[`ModelMesh`](crate::geometry::ModelMesh)**.
//!
//! Use **[`cube`]** plus **[`ModelMesh::transform`](crate::geometry::ModelMesh::transform)** for posing
//! (**`ModelMesh::transform`** / **`ModelMesh::visible_triangles`** — same **`Camera`** +**Z**‑forward semantics as the rest of the crate).

use glam::Vec3;

use mare_imbrium::geometry::{Facet, ModelMesh, UnitVec3};

/// Two **`Facet`**s per planar hull quad (same **`normal`**, **`(w,x,y)` + `(w,y,z)`** given CCW verts **`w…z`** seen from outside along **`normal`**).
const fn facets_from_quad_ccw_corner(normal: UnitVec3, verts: [usize; 4]) -> [Facet; 2] {
    let [w, x, y, z] = verts;
    [
        Facet::with_facet_normal([w, x, y], normal),
        Facet::with_facet_normal([w, y, z], normal),
    ]
}

const UNIT_CUBE_VERTICES: [Vec3; 8] = [
    Vec3::new(-0.5, -0.5, -0.5),
    Vec3::new(0.5, -0.5, -0.5),
    Vec3::new(0.5, 0.5, -0.5),
    Vec3::new(-0.5, 0.5, -0.5),
    Vec3::new(-0.5, -0.5, 0.5),
    Vec3::new(0.5, -0.5, 0.5),
    Vec3::new(0.5, 0.5, 0.5),
    Vec3::new(-0.5, 0.5, 0.5),
];

// Six hull quads (**CCW**) from historical **`cube`** vertex layout (**one normal + four corners**).
const UNIT_CUBE_QUADS: [(UnitVec3, [usize; 4]); 6] = [
    (UnitVec3::NEG_Z, [0, 3, 2, 1]),
    (UnitVec3::Z, [4, 5, 6, 7]),
    (UnitVec3::X, [1, 2, 6, 5]),
    (UnitVec3::NEG_X, [0, 4, 7, 3]),
    (UnitVec3::Y, [3, 7, 6, 2]),
    (UnitVec3::NEG_Y, [0, 1, 5, 4]),
];

/// Canonical axis-aligned **`[-½, ½]³`** mesh (**eight verts**, twelve wedge **`Facet`**s (**`(w,x,y)` **`(w,y,z)`** per planar quad)).
#[must_use]
pub fn cube() -> ModelMesh {
    let mut facets = Vec::with_capacity(12);
    for &(normal, corners) in &UNIT_CUBE_QUADS {
        let [a, b] = facets_from_quad_ccw_corner(normal, corners);
        facets.push(a);
        facets.push(b);
    }
    ModelMesh::new(UNIT_CUBE_VERTICES.into_iter().collect(), facets)
}

#[cfg(test)]
mod tests {
    use super::cube;

    #[test]
    fn cube_corner_and_facet_counts() {
        let mesh = cube();
        assert_eq!(mesh.vertices().len(), 8);
        assert_eq!(mesh.facets().len(), 12);
        assert!(
            mesh.vertices()
                .iter()
                .all(|p| p.x.abs() <= 0.5 && p.y.abs() <= 0.5 && p.z.abs() <= 0.5),
        );
    }
}
