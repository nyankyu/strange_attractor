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
    const ANGLE_OF_VIEW: f32 = 90.0 * (PI / 180.0);

    const ORBIT_NUM: usize = 1300;
    const ORBIT_LEN: usize = 500;
    const ORBIT_WEIGHT: f32 = 0.6;

    const DRAW_SKIP: usize = Self::ORBIT_LEN * 5;

    const DELTA_T: f32 = 0.002;

    const CAMERA: Vec3A = const_vec3a!([-7.0, 0.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([4.0, 0.0, 0.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 7.9;
    const ROTAION_Z: f32 = 5.9;

    const COLOR: Rgb8 = GOLD;

    fn new() -> Self {
        DadrasAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-5.0, 5.0),
            random_range(-5.0, 5.0),
            random_range(-5.0, 5.0),
        )
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = p.y - A * p.x + B * p.y * p.z;
        let dy = C * p.y + p.z * (1.0 - p.x);
        let dz = D * p.x * p.y - E * p.z;
        vec3a(dx, dy, dz)
    }
}
