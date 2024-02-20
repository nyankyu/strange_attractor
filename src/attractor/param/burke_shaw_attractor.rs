use crate::AttractorParam;
use nannou::glam::const_vec3a;
use nannou::prelude::*;

const S: f32 = 10.0;
const V: f32 = 4.272;

pub(crate) struct BurkeShawAttractor {}

impl AttractorParam for BurkeShawAttractor {
    const ANGLE_OF_VIEW: f32 = 90.0 / 180.0 * PI;

    const ORBIT_NUM: usize = 400;
    const ORBIT_LEN: usize = 700;
    const ORBIT_WEIGHT: f32 = 0.9;

    const DRAW_SKIP: usize = Self::ORBIT_LEN * 4;

    const DELTA_T: f32 = 0.001;

    const CAMERA: Vec3A = const_vec3a!([-3.0, 1.0, 0.0]);
    const CENTER: Vec3A = const_vec3a!([-0.5, 1.0, 0.0]);

    const DELTA_THETA: f32 = 0.0003;

    const ROTAION_X: f32 = -1.3;
    const ROTAION_Y: f32 = -7.9;
    const ROTAION_Z: f32 = 1.0;

    const COLOR: Rgb8 = MEDIUMAQUAMARINE;

    fn new() -> Self {
        BurkeShawAttractor {}
    }

    fn random_point() -> Vec3A {
        vec3a(
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
            random_range(-1.0, 1.0),
        )
    }

    fn slope(p: Vec3A) -> Vec3A {
        let dx = - S * (p.x + p.y);
        let dy = - p.y - S * p.x * p.z;
        let dz = S * p.x * p.y + V;
        vec3a(dx, dy, dz)
    }
}
