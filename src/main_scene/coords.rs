use glam::Vec3;

pub fn to_unit_spherical(p: Vec3) -> (f32, f32) {
    debug_assert!(p.is_normalized());

    let azimuth = p.z.atan2(p.x);
    let polar = p.y.acos();
    (azimuth, polar)
}
