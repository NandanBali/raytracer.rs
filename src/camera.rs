use std::fs::File;

use crate::object::World;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct Camera {
    file: File,
    world: World,
}

impl Camera {
    pub fn new(file: File, world: World) -> Self {
        Camera { file, world }
    }
    pub fn render(&mut self, image_height: usize, image_width: usize) {
        let viewport_height: f64 = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        let (focal_length, camera_center) = (1.0, Vec3(0.0, 0.0, 0.0));
        let (viewport_v, viewport_u) = (
            Vec3(0.0, -viewport_height, 0.0),
            Vec3(viewport_width, 0.0, 0.0),
        );

        let (delta_v, delta_u) = (
            viewport_v / image_height as f64,
            viewport_u / image_width as f64,
        );

        let viewport_upper_left =
            camera_center - Vec3(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel_offset = viewport_upper_left + (delta_u + delta_v) * 0.5;

        for y in 0..image_height {
            for x in 0..image_width {
                let pixel_center = (pixel_offset) + (delta_u * x as f64) + (delta_v * y as f64);
                let ray = Ray {
                    origin: camera_center,
                    direction: pixel_center - camera_center,
                };
                let color = self.color_at(&ray);
                Color::write_color(&mut self.file, color);
            }
        }
    }

    fn color_at(&self, ray: &Ray) -> Color {
        // declare sphere
        if let Some(res) = self.world.hit(ray, (0., f64::INFINITY)) {
            return (res.normal + 1.) * 0.5;
        }
        let ht = 0.5 * (ray.direction.unit().1 + 1.);
        (Vec3(1., 1., 1.) * (1. - ht)) + (Vec3(0.5, 0.7, 1.) * ht)
    }

    fn antialiasing_ray(i: u16, j: u16, del_u: f64, del_v: f64) {
        unimplemented!()
    }

    fn sample_square() -> Vec3 {
        Vec3(
            rand::random_range(-0.5..0.5),
            rand::random_range(-0.5..0.5),
            0.,
        )
    }
}
