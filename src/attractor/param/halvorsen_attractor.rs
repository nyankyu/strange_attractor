use nannou::prelude::*;
use nannou::glam::const_mat3a;
use nannou::glam::const_vec3a;
use crate::AttractorParam;

const A: f32 = 1.3;
const MAT: Mat3A = const_mat3a!([-A, -4.0, -4.0], [-4.0, -A, -4.0], [-4.0, -4.0, -A]);

pub(crate) struct HalvorsenAttractor {}

impl AttractorParam for HalvorsenAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 * (PI / 180.0);

    const ORBIT_NUM: usize = 2700;
    const ORBIT_LEN: usize = 300;
    const ORBIT_WEIGHT: f32 = 0.8;

    const DRAW_SKIP: usize = Self::ORBIT_LEN * 8;

    const DELTA_T: f32 = 0.001;

    const CAMERA: Vec3A = const_vec3a!([-10.0, -5.0, -5.0]);
    const CENTER: Vec3A = const_vec3a!([3.0, -6.0, -6.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = -7.0;
    const ROTAION_Y: f32 = 3.3;
    const ROTAION_Z: f32 = 1.3;

    const COLOR: Rgb8 = DODGERBLUE;

    fn new() -> Self {
        HalvorsenAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
        )
    }

    fn make_next(p: &Vec3A) -> Vec3A {
        let d = MAT * *p - vec3a(p.y * p.y, p.z * p.z, p.x * p.x);
        *p + d * Self::DELTA_T
    }
}
