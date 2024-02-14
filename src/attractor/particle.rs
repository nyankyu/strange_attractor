use std::{collections::VecDeque, marker::PhantomData};

use nannou::{glam::Vec3Swizzles, prelude::*};

use super::AttractorParam;

pub(super) struct Particle<Param: AttractorParam> {
    _param: PhantomData<fn() -> Param>,
    orbit: VecDeque<Vec3A>,
}

impl<Param: AttractorParam> Particle<Param> {
    pub(super) fn new() -> Self {
        let mut list = VecDeque::with_capacity(Param::ORBIT_LEN + 1);
        list.push_back(Param::random_point());

        Particle {
            _param: PhantomData,
            orbit: list,
        }
    }

    pub(super) fn update(&mut self) {
        let next = Self::runge_kutta4(*self.orbit.back().unwrap());
        self.orbit.push_back(next);

        if self.orbit.len() > Param::ORBIT_LEN {
            self.orbit.pop_front();
        }
    }

    pub(super) fn draw(&self, draw: &Draw, rotation: Quat, distance_screen: f32) {
        draw.polyline()
            .weight(Param::ORBIT_WEIGHT)
            .join_round()
            .points_colored(self.orbit.iter().map(|&p| {
                let rotated = rotation.mul_vec3a(p - Param::CENTER) + Param::CENTER;
                let depth = rotated.x - Param::CAMERA.x;
                let (scale, alpha) = if depth > 0.0 {
                    (distance_screen / depth, 180)
                } else {
                    (1000_000.0, 0)
                };
                let coordinate = (rotated.yz() - Param::CAMERA.yz()) * scale;
                let color = Rgba8 {
                    color: Param::COLOR,
                    alpha,
                };
                (coordinate, color)
            }));
    }

    fn runge_kutta4(p: Vec3A) -> Vec3A {
        let k1 = Param::DELTA_T * Param::slope(p);
        let k2 = Param::DELTA_T * Param::slope(p + 0.5 * k1);
        let k3 = Param::DELTA_T * Param::slope(p + 0.5 * k2);
        let k4 = Param::DELTA_T * Param::slope(p + k3);
        p + (k1 + k2 + k2 + k3 + k3 + k4) / 6.0
    }
}
