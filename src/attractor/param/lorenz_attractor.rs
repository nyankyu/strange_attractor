use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const SIGMA: f32 = 10.0;
const BETA: f32 = 8.0 / 3.0;
const RHO: f32 = 28.0;

pub(crate) struct LorenzAttractor {}

impl AttractorParam for LorenzAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 / 180.0 * PI;

    const ORBIT_NUM: usize = 800;
    const ORBIT_LEN: usize = 600;
    const ORBIT_WEIGHT: f32 = 0.5;

    const DRAW_SKIP: usize = Self::ORBIT_LEN * 2;

    const DELTA_T: f32 = 0.001;

    const CAMERA: Vec3A = const_vec3a!([-30.0, 10.0, 20.0]);
    const CENTER: Vec3A = const_vec3a!([0.0, 10.0, 20.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = -1.3;
    const ROTAION_Y: f32 = -7.9;
    const ROTAION_Z: f32 = 1.0;

    const COLOR: Rgb8 = RED;

    fn new() -> Self {
        LorenzAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-30.0, 30.0),
            random_range(-30.0, 30.0),
            random_range(0.0, 60.0),
        )
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = SIGMA * (p.y - p.x);
        let dy = p.x * (RHO - p.z) - p.y;
        let dz = p.x * p.y - BETA * p.z;
        vec3a(dx, dy, dz)
    }
}
