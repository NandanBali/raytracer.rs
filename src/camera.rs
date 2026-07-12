use std::fs::File;

use crate::object::World;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

struct CameraInfo {
    image_height: i32,
    image_width: i32,
    center: Vec3, // camera center
    starting_point: Vec3, // location of 0, 0
    delta_u: Vec3,
    delta_v: Vec3
}

pub struct Camera {
    file: File,
    world: World,
    info: CameraInfo
}

impl Camera {
    pub fn new(file: File, world: World, image_height: usize, image_width: usize) -> Self {
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

        let info = CameraInfo { image_height: image_height as i32, image_width: image_width as i32, center: camera_center, starting_point: pixel_offset, delta_u, delta_v };


        Camera { file, world, info: info}
    }
    pub fn render(&mut self) {
       for y in 0..self.info.image_height {
            for x in 0..self.info.image_width {
               let mut color = Vec3 (0., 0., 0.);
                for _ in 0..10 {
                    let r = self.antialiasing_ray(x, y);
                    color = color + self.color_at(&r, 50);
                }
                color = color / 10.;
                Color::write_color(&mut self.file, color);
            }
        }
    }

    fn color_at(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Vec3(0., 0., 0.);
        }
        if let Some(res) = self.world.hit(ray, (0.0001, f64::INFINITY)) {
            if let Some((scatter, attenuation)) = res.material.scatter(ray, &res) {
                return Vec3::stmul(self.color_at(&scatter, depth + 1), attenuation);
            }
            return Vec3(0., 0., 0.);
        }
        let ht = 0.5 * (ray.direction.unit().1 + 1.);
        (Vec3(1., 1., 1.) * (1. - ht)) + (Vec3(0.5, 0.7, 1.) * ht)
    }

    fn antialiasing_ray(&self, i: i32, j: i32) -> Ray {
        
        let offset = Camera::sample_square();
        let pixel_sample = self.info.starting_point + (self.info.delta_u * (i as f64 + offset.0)) + (self.info.delta_v * (j as f64 + offset.1));
        Ray {origin: self.info.center, direction: pixel_sample - self.info.center}
    }

    fn sample_square() -> Vec3 {
        
        Vec3(
            rand::random_range(-0.5..0.5),
            rand::random_range(-0.5..0.5),
            0.,
        )
    }
}
