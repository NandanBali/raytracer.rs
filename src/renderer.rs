use crate::camera::Camera;
use crate::object::World;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::fs::File;

pub trait Renderer {
    fn new(world: World, camera: Camera) -> Self;
    fn render(&self, output: &mut File);
}

pub struct SimpleRenderer {
    world: World,
    camera: Camera,
}

impl SimpleRenderer {
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
}
impl Renderer for SimpleRenderer {
    fn new(world: World, camera: Camera) -> Self {
        SimpleRenderer { world, camera }
    }

    fn render(&self, output: &mut File) {
        for y in 0..self.camera.image_height {
            for x in 0..self.camera.image_width {
                let mut color = Vec3(0., 0., 0.);
                for _ in 0..30 {
                    let r = self.camera.antialiasing(x, y);
                    color = color + self.color_at(&r, 50);
                }
                color = color / 30.;
                Color::write_color(output, color);
            }
        }
    }
}

pub struct ParallelRenderer {
    world: World,
    camera: Camera,
}

impl ParallelRenderer {
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

    fn render_pixel(&self, x: i32, y: i32) -> Color {
        let mut color = Vec3(0., 0., 0.);
        for _ in 0..30 {
            let r = self.camera.antialiasing(x, y);
            color = color + self.color_at(&r, 50);
        }
        color = color / 30.;
        color
    }
}
impl Renderer for ParallelRenderer {
    fn new(world: World, camera: Camera) -> Self {
        ParallelRenderer { world, camera }
    }

    fn render(&self, output: &mut File) {
        let coords: Vec<(i32, i32)> = (0..self.camera.image_height)
            .flat_map(|y| (0..self.camera.image_width).map(move |x| (x, y)))
            .collect();
        let results = coords.into_par_iter().map(move |(x,y)| {
            self.render_pixel(x, y)
       }).collect::<Vec<Vec3>>();

        println!("writing to file now");
        for color in results {
            Color::write_color(output, color);
        }
    }
}
