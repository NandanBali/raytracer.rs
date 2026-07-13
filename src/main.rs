use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::renderer::{ParallelRenderer, Renderer, SimpleRenderer};
use crate::{
    camera::Camera,
    object::{Sphere, World},
    vec3::Vec3,
};
use rand::random_range;
use std::sync::Arc;
use std::{fs::File, fs::OpenOptions, io::Write};

mod camera;
mod helpers;
mod material;
mod object;
mod ray;
mod vec3;
mod renderer;

const PATH: &str = "output.ppm";
const ASPECT_RATIO: f64 = 16_f64 / 10_f64;
const IMG_WIDTH: u32 = 1440;
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

fn add_random_spheres(world: &mut World) {
    for a in -11..11 {
        for b in -11..11 {
            let center = Vec3(
                a as f64 + random_range(0. ..0.9),
                0.2,
                b as f64 + random_range(0. ..0.9),
            );
            // keep the small spheres clear of the big metal one
            if (center - Vec3(4., 0.2, 0.)).length() <= 0.9 {
                continue;
            }

            let choose = random_range(0. ..1.);
            let material: Arc<dyn Material + Send + Sync> = if choose < 0.8 {
                Arc::new(Lambertian {
                    albedo: Vec3::stmul(Vec3::random(), Vec3::random()),
                })
            } else if choose < 0.95 {
                Arc::new(Metal {
                    albedo: Vec3(
                        random_range(0.5..1.),
                        random_range(0.5..1.),
                        random_range(0.5..1.),
                    ),
                    fuzz: random_range(0. ..0.5),
                })
            } else {
                Arc::new(Dielectric {
                    refraction_index: 1.5,
                })
            };

            world.add_obj(Box::new(Sphere {
                center,
                radius: 0.2,
                material,
            }));
        }
    }
}

fn main() {
    let mut file = init_file();
    let mut world = World::new();

    world.add_obj(Box::new(Sphere {
        center: Vec3(0., -1000., 0.),
        radius: 1000.,
        material: Arc::new(Lambertian {albedo: Vec3(0.33, 0.33, 0.67)}),
    }));

    add_random_spheres(&mut world);

    world.add_obj(Box::new(Sphere {
        center: Vec3(0., 1., 0.),
        radius: 1.,
        material: Arc::new(Dielectric {
            refraction_index: 1.5,
        }),
    }));
    world.add_obj(Box::new(Sphere {
        center: Vec3(-4., 1., 0.),
        radius: 1.,
        material: Arc::new(Lambertian {
            albedo: Vec3(0.4, 0.2, 0.1),
        }),
    }));
    world.add_obj(Box::new(Sphere {
        center: Vec3(4., 1., 0.),
        radius: 1.,
        material: Arc::new(Metal {
            albedo: Vec3(0.7, 0.6, 0.5),
            fuzz: 0.,
        }),
    }));

    let camera = Camera::new(
        IMAGE_HEIGHT as usize,
        IMG_WIDTH as usize,
        20.,
        Vec3(13., 2., 3.),
        Vec3(0., 0., 0.),
        Vec3(0., 1., 0.),
        0.6,
        10.,
    );

    let renderer = ParallelRenderer::new(world, camera);
    renderer.render(&mut file);
}
