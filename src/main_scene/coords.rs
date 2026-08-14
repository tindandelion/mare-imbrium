use std::f32::consts;

use glam::Vec3;

pub fn to_spherical(normal: Vec3) -> (f32, f32) {
    debug_assert!(normal.is_normalized());

    let polar = normal.y.acos();
    let azimuth = {
        let angle = -normal.x.atan2(normal.z);
        if normal.x < 0.0 {
            angle
        } else {
            consts::TAU - 1e-6 + angle
        }
    };

    (azimuth, polar)
}

#[cfg(test)]
mod tests {
    use std::f32::consts;

    use approx::assert_relative_eq;
    use glam::Vec3;

    use super::*;

    #[test]
    fn polar_angle_changes_from_north_pole_to_south_pole() {
        // North pole: Y axis
        let (_, polar) = to_spherical(Vec3::new(0.0, 1.0, 0.0));
        assert_relative_eq!(0.0, polar);

        let (_, polar) = to_spherical(Vec3::new(0.0, 0.5, -0.5).normalize());
        assert_relative_eq!(consts::FRAC_PI_4, polar);

        let (_, polar) = to_spherical(Vec3::new(0.0, 0.0, -1.0).normalize());
        assert_relative_eq!(consts::FRAC_PI_2, polar);

        let (_, polar) = to_spherical(Vec3::new(0.0, -0.5, -0.5).normalize());
        assert_relative_eq!(consts::FRAC_PI_2 + consts::FRAC_PI_4, polar);

        // South pole: -Y axis
        let (_, polar) = to_spherical(Vec3::new(0.0, -1.0, 0.0).normalize());
        assert_relative_eq!(consts::PI, polar);
    }

    #[test]
    fn azimuth_angle_changes_around_the_equator() {
        // Start: Z axis
        let (azimuth, _) = to_spherical(Vec3::new(-1e-7, 0.0, 1.0).normalize());
        assert_relative_eq!(0.0, azimuth, epsilon = 1e-6);

        let (azimuth, _) = to_spherical(Vec3::new(-0.5, 0.0, 0.5).normalize());
        assert_relative_eq!(consts::FRAC_PI_4, azimuth, epsilon = 1e-6);

        let (azimuth, _) = to_spherical(Vec3::new(-1.0, 0.0, 0.0).normalize());
        assert_relative_eq!(consts::FRAC_PI_2, azimuth, epsilon = 1e-6);

        let (azimuth, _) = to_spherical(Vec3::new(-0.5, 0.0, -0.5).normalize());
        assert_relative_eq!(
            consts::FRAC_PI_2 + consts::FRAC_PI_4,
            azimuth,
            epsilon = 1e-6
        );

        let (azimuth, _) = to_spherical(Vec3::new(0.0, 0.0, -1.0).normalize());
        assert_relative_eq!(consts::PI, azimuth, epsilon = 1e-6);

        let (azimuth, _) = to_spherical(Vec3::new(0.5, 0.0, -0.5).normalize());
        assert_relative_eq!(consts::PI + consts::FRAC_PI_4, azimuth, epsilon = 1e-6);

        let (azimuth, _) = to_spherical(Vec3::new(1.0, 0.0, 0.0).normalize());
        assert_relative_eq!(consts::PI + consts::FRAC_PI_2, azimuth, epsilon = 1e-6);

        let (azimuth, _) = to_spherical(Vec3::new(0.5, 0.0, 0.5).normalize());
        assert_relative_eq!(
            consts::PI + consts::FRAC_PI_2 + consts::FRAC_PI_4,
            azimuth,
            epsilon = 1e-6
        );

        let (azimuth, _) = to_spherical(Vec3::new(1e-7, 0.0, 1.0).normalize());
        assert_relative_eq!(consts::TAU, azimuth, epsilon = 1e-6);

        let (azimuth, _) = to_spherical(Vec3::new(0.0, 0.0, 1.0).normalize());
        assert_relative_eq!(consts::TAU, azimuth, epsilon = 1e-6);
    }
}
