use std::f32::consts;

use glam::Vec3;

pub struct TextureCoordMapper {
    width: usize,
    height: usize,
}

impl TextureCoordMapper {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn to_uv_coords(&self, normal: Vec3) -> (usize, usize) {
        let (azimuth, polar) = Self::to_latlon(normal);
        self.polar_to_uv(azimuth, polar)
    }

    fn to_latlon(normal: Vec3) -> (f32, f32) {
        debug_assert!(normal.is_normalized());

        let polar = normal.y.acos();
        let azimuth = normal.x.atan2(-normal.z) + consts::PI;

        (
            azimuth.min(consts::TAU - 1e-6),
            polar.min(consts::PI - 1e-6),
        )
    }

    fn polar_to_uv(&self, azimuth: f32, polar: f32) -> (usize, usize) {
        let u = (self.width as f32 * azimuth / consts::TAU) as usize;
        let v = (self.height as f32 * polar / consts::PI) as usize;
        (u, v)
    }
}

#[cfg(test)]
mod tests {
    use glam::Vec3;

    use super::*;

    #[test]
    fn polar_angle_changes_from_north_pole_to_south_pole() {
        let mapper = TextureCoordMapper::new(1024, 512);
        // North pole: Y axis
        let (_, polar) = mapper.to_uv_coords(Vec3::new(0.0, 1.0, 0.0));
        assert_eq!(0, polar);

        let (_, polar) = mapper.to_uv_coords(Vec3::new(0.0, 0.5, -0.5).normalize());
        assert_eq!(128, polar);

        let (_, polar) = mapper.to_uv_coords(Vec3::new(0.0, 0.0, -1.0).normalize());
        assert_eq!(256, polar);

        let (_, polar) = mapper.to_uv_coords(Vec3::new(0.0, -0.5, -0.5).normalize());
        assert_eq!(384, polar);

        // South pole: -Y axis
        let (_, polar) = mapper.to_uv_coords(Vec3::new(0.0, -1.0, 0.0).normalize());
        assert_eq!(511, polar);
    }

    #[test]
    fn azimuth_angle_changes_around_the_equator() {
        let mapper = TextureCoordMapper::new(1024, 512);
        // Start: Z axis
        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(-1e-7, 0.0, 1.0).normalize());
        assert_eq!(0, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(-0.5, 0.0, 0.5).normalize());
        assert_eq!(128, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(-1.0, 0.0, 0.0).normalize());
        assert_eq!(256, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(-0.5, 0.0, -0.5).normalize());
        assert_eq!(384, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(0.0, 0.0, -1.0).normalize());
        assert_eq!(512, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(0.5, 0.0, -0.5).normalize());
        assert_eq!(640, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(1.0, 0.0, 0.0).normalize());
        assert_eq!(768, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(0.5, 0.0, 0.5).normalize());
        assert_eq!(896, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(1e-7, 0.0, 1.0).normalize());
        assert_eq!(1023, azimuth);

        let (azimuth, _) = mapper.to_uv_coords(Vec3::new(0.0, 0.0, 1.0).normalize());
        assert_eq!(1023, azimuth);
    }
}
