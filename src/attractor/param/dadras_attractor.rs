use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const A: f32 = 3.0;
const B: f32 = 2.7;
const C: f32 = 1.7;
const D: f32 = 2.0;
const E: f32 = 9.0;

pub(crate) struct DadrasAttractor {}

impl AttractorParam for DadrasAttractor {
    const ANGLE_OF_VIEW: f32 = 80.0 * (PI / 180.0);

    const ORBIT_NUM: usize = 300;
    const ORBIT_LEN: usize = 400;
    const ORBIT_WEIGHT: f32 = 1.0;

    const DRAW_SKIP: usize = Self::ORBIT_LEN * 2;

    const DELTA_T: f32 = 0.001;

    const CAMERA: Vec3A = const_vec3a!([-20.0, 0.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([0.0, 0.0, 0.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 7.9;
    const ROTAION_Z: f32 = 1.9;

    const COLOR: Rgb8 = DODGERBLUE;

    fn new() -> Self {
        DadrasAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-40.0, 40.0),
            random_range(-40.0, 40.0),
            random_range(-40.0, 40.0),
        )
    }

    fn make_next(p: &Vec3A) -> Vec3A {
        let dx = p.y - A * p.x + B * p.y * p.z;
        let dy = C * p.y + p.z * (1.0 - p.x);
        let dz = D * p.x * p.y - E * p.z;
        *p + vec3a(dx, dy, dz) * Self::DELTA_T
    }
}
