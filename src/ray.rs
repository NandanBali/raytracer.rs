use crate::{object::World, vec3::Color};

use super::vec3::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + (self.direction * t)
    }

    pub fn color_at(&self, world: &World) -> Color {
        // declare sphere
        if let Some(res) = world.hit(self, (0., f64::INFINITY)) {
            return (res.normal + 1.) * 0.5;
        }
        let ht = 0.5 * (self.direction.unit().1 + 1.);
        (Vec3(1., 1., 1.) * (1. - ht)) + (Vec3(0.5, 0.7, 1.) * ht)
    }
}
