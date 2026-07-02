use std::{fs::File, fs::OpenOptions, io::Write};

use crate::{
    camera::Camera,
    object::{Sphere, World},
    vec3::Vec3,
};

mod camera;
mod helpers;
mod object;
mod ray;
mod vec3;
const PATH: &str = "output.ppm";
const ASPECT_RATIO: f64 = 16_f64 / 9_f64;
const IMG_WIDTH: u32 = 512;
const IMAGE_HEIGHT: u32 = (IMG_WIDTH as f64 / ASPECT_RATIO) as u32;

fn init_file() -> File {
    let file_header = format!("P3\n{} {}\n255\n", IMG_WIDTH, IMAGE_HEIGHT);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(PATH)
        .expect("Couldn't write to file");

    let _ = writeln!(file, "{}", file_header);
    file
}

fn main() {
    let file = init_file();
    let mut world = World::new();

    world.add_obj(Box::new(Sphere {
        center: Vec3(0., 0., -1.),
        radius: 0.35,
    }));

    world.add_obj(Box::new(Sphere {
        center: Vec3(0., -100.5, -1.),
        radius: 100.,
    }));

    let mut camera = Camera::new(file, world);
    camera.render(IMAGE_HEIGHT as usize, IMG_WIDTH as usize);
}
