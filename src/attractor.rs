mod param;
mod particle;

use std::marker::PhantomData;
use nannou::{glam::{EulerRot, Vec3Swizzles}, prelude::*};

use crate::WINDOW_W;

#[allow(unused_imports)]
pub(crate) use param::lorenz_attractor::LorenzAttractor;
#[allow(unused_imports)]
pub(crate) use param::halvorsen_attractor::HalvorsenAttractor;
#[allow(unused_imports)]
pub(crate) use param::thomas_attractor::ThomasAttractor;
#[allow(unused_imports)]
pub(crate) use param::langford_attractor::LangfordAttractor;
#[allow(unused_imports)]
pub(crate) use param::dadras_attractor::DadrasAttractor;
#[allow(unused_imports)]
pub(crate) use param::lorenz83_attractor::Lorenz83Attractor;

use particle::Particle;

pub(crate) trait AttractorParam {
    const ANGLE_OF_VIEW: f32;

    const ORBIT_NUM: usize;
    const ORBIT_LEN: usize;
    const ORBIT_WEIGHT: f32;

    const DRAW_SKIP: usize;

    const DELTA_T: f32;

    /// direction right: +y, left: -y, top: +z, bottom: -z, front: +x, back: -x
    const CAMERA: Vec3A;
    const CENTER: Vec3A;

    const DELTA_THETA: f32;

    const ROTAION_X: f32;
    const ROTAION_Y: f32;
    const ROTAION_Z: f32;

    const COLOR: Rgb8;

    fn new() -> Self;
    fn random_point() -> Vec3A;
    fn make_next(p: &Vec3A) -> Vec3A;
}

pub(crate) struct Attractor<Param: AttractorParam> {
    _param: PhantomData<fn() -> Param>,
    orbits: Vec<Particle<Param>>,
    theta: f32,
    rotation: Quat,
    distance_screen: f32,
}

impl<Param: AttractorParam> Attractor<Param> {
    pub(crate) fn new() -> Self {
        Attractor {
            _param: PhantomData,
            orbits: (0..Param::ORBIT_NUM)
                .map(|_| {
                    let mut particle = Particle::new();
                    for _ in 0..Param::DRAW_SKIP {
                        particle.update();
                    }
                    particle
                })
                .collect(),
            theta: 0.0,
            rotation: Quat::IDENTITY,
            distance_screen: WINDOW_W as f32 * 0.5 / (0.5 * Param::ANGLE_OF_VIEW).tan(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.orbits.iter_mut().for_each(|p| p.update());
        self.theta += Param::DELTA_THETA;
        self.rotation = Quat::from_euler(
            EulerRot::ZYX,
            self.theta * Param::ROTAION_X,
            self.theta * Param::ROTAION_Y,
            self.theta * Param::ROTAION_Z,
        );
    }
    pub(crate) fn draw(&self, draw: &Draw) {
        self.orbits
            .iter()
            .for_each(|particle| particle.draw(&draw, self.rotation, self.distance_screen));
        if !crate::RECORDING {
            self.draw_axis(&draw);
        }
    }

    fn draw_axis(&self, draw: &Draw) {
        draw.line()
            .weight(5.0)
            .color(WHITE)
            .end(self.coordinate(Param::CENTER + Vec3A::AXES[0]))
            .start(self.coordinate(Param::CENTER));
        draw.line()
            .weight(5.0)
            .color(BLUE)
            .end(self.coordinate(Param::CENTER + Vec3A::AXES[1]))
            .start(self.coordinate(Param::CENTER));
        draw.line()
            .weight(5.0)
            .color(GREEN)
            .end(self.coordinate(Param::CENTER + Vec3A::AXES[2]))
            .start(self.coordinate(Param::CENTER));
        draw.ellipse()
            .radius(5.0)
            .color(WHITE)
            .xy(self.coordinate(Param::CENTER));
    }

    fn coordinate(&self, p: Vec3A) -> Vec2 {
        let rotated = self.rotation * (p - Param::CENTER) + Param::CENTER;
        let depth = rotated.x - Param::CAMERA.x;
        let scale = self.distance_screen / depth;
        (rotated.yz() - Param::CAMERA.yz()) * scale - Param::CAMERA.yz()
    }
}
