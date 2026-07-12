use std::fs::File;

use crate::object::World;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

struct CameraInfo {
    image_height: i32,
    image_width: i32,
    center: Vec3,         // camera center
    starting_point: Vec3, // location of 0, 0
    delta_u: Vec3,
    delta_v: Vec3,
    fov: f64,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

pub struct Camera {
    file: File,
    world: World,
    info: CameraInfo,
}

impl Camera {
    fn degree_to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    pub fn new(
        file: File,
        world: World,
        image_height: usize,
        image_width: usize,
        fov: f64,
        cfrom: Vec3,
        cto: Vec3,
        vup: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let tan = |x: f64| {x.sin() / x.cos()};
        let viewport_height: f64 = tan(Self::degree_to_radians(fov) / 2.) * 2. * focus_dist;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // orthonormal camera basis: w points opposite the view direction,
        // u points to the camera's right, v points up in the frame
        let w = (cfrom - cto).unit();
        let u = Vec3::cross(vup, w).unit();
        let v = Vec3::cross(w, u);

        let (viewport_v, viewport_u) = (
            v * -viewport_height,
            u * viewport_width,
        );

        let (delta_v, delta_u) = (
            viewport_v / image_height as f64,
            viewport_u / image_width as f64,
        );

        let viewport_upper_left =
            cfrom - w * focus_dist - viewport_u / 2. - viewport_v / 2.;
        let pixel_offset = viewport_upper_left + (delta_u + delta_v) * 0.5;

        let defocus_radius = focus_dist * tan(Self::degree_to_radians(defocus_angle / 2.));
        let (defocus_disk_u, defocus_disk_v) = (u * defocus_radius, v * defocus_radius);

        let info = CameraInfo {
            image_height: image_height as i32,
            image_width: image_width as i32,
            center: cfrom,
            starting_point: pixel_offset,
            delta_u,
            delta_v,
            fov,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
        };
        Camera {
            file,
            world,
            info: info,
        }
    }
    pub fn render(&mut self) {
        for y in 0..self.info.image_height {
            for x in 0..self.info.image_width {
                let mut color = Vec3(0., 0., 0.);
                for _ in 0..100 {
                    let r = self.antialiasing_ray(x, y);
                    color = color + self.color_at(&r, 50);
                }
                color = color / 100.;
                Color::write_color(&mut self.file, color);
            }
        }
    }

    fn color_at(&self, ray: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Vec3(0., 0., 0.);
        }
        if let Some(res) = self.world.hit(ray, (0.001, f64::INFINITY)) {
            if let Some((scatter, attenuation)) = res.material.scatter(ray, &res) {
                return Vec3::stmul(self.color_at(&scatter, depth - 1), attenuation);
            }
            return Vec3(0., 0., 0.);
        }
        let ht = 0.5 * (ray.direction.unit().1 + 1.);
        (Vec3(1., 1., 1.) * (1. - ht)) + (Vec3(0.5, 0.7, 1.) * ht)
    }

    fn antialiasing_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.info.starting_point
            + (self.info.delta_u * (i as f64 + offset.0))
            + (self.info.delta_v * (j as f64 + offset.1));
        let origin = if self.info.defocus_angle <= 0. {
            self.info.center
        } else {
            self.defocus_disk_sample()
        };
        Ray {
            origin,
            direction: pixel_sample - origin,
        }
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.info.center + (self.info.defocus_disk_u * p.0) + (self.info.defocus_disk_v * p.1)
    }

    fn sample_square() -> Vec3 {
        Vec3(
            rand::random_range(-0.5..0.5),
            rand::random_range(-0.5..0.5),
            0.,
        )
    }
}
