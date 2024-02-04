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
        let last = Param::make_next(&self.orbit.back().unwrap());
        self.orbit.push_back(last);

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
                    let distance = Param::CAMERA.distance(rotated);
                    (
                        distance_screen / depth,
                        255 - (distance as u8).saturating_mul(4),
                    )
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
}
