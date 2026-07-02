use std::{fs::OpenOptions, io::Write};

use crate::{
    object::{Sphere, World},
    ray::Ray,
    vec3::{Color, Vec3},
};

mod object;
mod ray;
mod vec3;
const PATH: &str = "output.ppm";
const ASPECT_RATIO: f64 = 16_f64 / 9_f64;

fn main() {
    let image_width = 512;
    let image_height = (image_width as f64 / ASPECT_RATIO) as u32;

    let file_header = format!("P3\n{} {}\n255\n", image_width, image_height);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(PATH)
        .expect("Couldn't write to file");

    let _ = writeln!(file, "{}", file_header);

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

    // world
    let mut world = World::new();

    world.add_obj(Box::new(Sphere {
        center: Vec3(0., 0., -1.),
        radius: 0.35,
    }));

    world.add_obj(Box::new(Sphere {
        center: Vec3(0., -100.5, -1.),
        radius: 100.,
    }));

    for y in 0..image_height {
        for x in 0..image_width {
            let pixel_center = (pixel_offset) + (delta_u * x as f64) + (delta_v * y as f64);
            let ray = Ray {
                origin: camera_center,
                direction: pixel_center - camera_center,
            };
            Color::write_color(&mut file, ray.color_at(&world));
        }
    }
}
