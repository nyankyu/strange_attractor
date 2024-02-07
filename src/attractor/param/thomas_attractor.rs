use nannou::prelude::*;
use nannou::glam::const_vec3a;
use crate::AttractorParam;

const B: f32 = 0.195;

pub(crate) struct ThomasAttractor {}

impl AttractorParam for ThomasAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 * (PI / 180.0);

    const ORBIT_NUM: usize = 5000;
    const ORBIT_LEN: usize = 100;
    const DRAW_SKIP: usize = Self::ORBIT_LEN * 7;
    const ORBIT_WEIGHT: f32 = 0.7;

    const DELTA_T: f32 = 0.02;

    const CAMERA: Vec3A = const_vec3a!([-3.0, 0.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([1.5, 0.0, 1.5]);

    const DELTA_THETA: f32 = 0.0004;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 7.9;
    const ROTAION_Z: f32 = 1.3;

    const COLOR: Rgb8 = LIMEGREEN;

    fn new() -> Self {
        ThomasAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-4.0, 4.0),
            random_range(-4.0, 4.0),
            random_range(-4.0, 4.0),
        )
    }

    fn make_next(p: &Vec3A) -> Vec3A {
        let dx = p.y.sin() - B * p.x;
        let dy = p.z.sin() - B * p.y;
        let dz = p.x.sin() - B * p.z;
        *p + vec3a(dx, dy, dz) * Self::DELTA_T
    }
}
