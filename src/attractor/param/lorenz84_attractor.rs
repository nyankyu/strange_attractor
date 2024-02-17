use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const A: f32 = 0.25;
const B: f32 = 4.0;
const F: f32 = 8.0;
const G: f32 = 1.0;

pub(crate) struct Lorenz84Attractor {}

impl AttractorParam for Lorenz84Attractor {
    const ANGLE_OF_VIEW: f32 = 100.0 / 180.0 * PI;

    const ORBIT_NUM: usize = 900;
    const ORBIT_LEN: usize = 200;
    const ORBIT_WEIGHT: f32 = 0.7;

    const DRAW_SKIP: usize = Self::ORBIT_LEN * 200;

    const DELTA_T: f32 = 0.003;

    const CAMERA: Vec3A = const_vec3a!([-2.0, 1.2, 0.0]);
    const CENTER: Vec3A = const_vec3a!([1.0, 1.5, 0.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = -1.3;
    const ROTAION_Y: f32 = -7.9;
    const ROTAION_Z: f32 = 1.0;

    const COLOR: Rgb8 = DEEPPINK;

    fn new() -> Self {
        Lorenz84Attractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(0.0, 3.0),
            random_range(-3.0, 3.0),
            random_range(0.0, 3.0),
        )
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = -A * p.x - p.y * p.y - p.z * p.z + A * F;
        let dy = -p.y + p.x * p.y - B * p.x * p.z + G;
        let dz = -p.z + B * p.x * p.y + p.x * p.z;
        vec3a(dx, dy, dz)
    }
}
