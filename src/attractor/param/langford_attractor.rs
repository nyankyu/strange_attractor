use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const A: f32 = 0.99;
const B: f32 = 0.72;
const C: f32 = 0.6;
const D: f32 = 3.5;
const E: f32 = 0.25;
const F: f32 = 0.2;

pub(crate) struct LangfordAttractor {}

impl AttractorParam for LangfordAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 * (PI / 180.0);

    const ORBIT_NUM: usize = 700;
    const ORBIT_LEN: usize = 300;
    const ORBIT_WEIGHT: f32 = 0.8;

    const DRAW_SKIP: usize = Self::ORBIT_LEN * 0;

    const DELTA_T: f32 = 0.004;

    const CAMERA: Vec3A = const_vec3a!([-1.5, 0.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([0.5, 0.5, 0.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = 1.0;
    const ROTAION_Y: f32 = 7.9;
    const ROTAION_Z: f32 = 1.9;

    const COLOR: Rgb8 = BLUEVIOLET;

    fn new() -> Self {
        LangfordAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-0.5, 0.5),
            random_range(-0.5, 0.5),
            random_range(-0.5, 0.5),
        )
    }

    fn make_next(p: &Vec3A) -> Vec3A {
        let dx = (p.z - B) * p.x - D * p.y;
        let dy = D * p.x + (p.z - B) * p.y;
        let dz = C + A * p.z - p.z * p.z * p.z / 3.0 - (p.x * p.x + p.y * p.y) * (1.0 + E * p.z)
            + F * p.z * p.x * p.x * p.x;
        *p + vec3a(dx, dy, dz) * Self::DELTA_T
    }
}
