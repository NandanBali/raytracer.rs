use crate::helpers::clamp;
use crate::material;
use crate::object::{HitData, Sphere};
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};
use std::rc::Rc;
use std::sync::Arc;

pub trait Material {
    fn scatter(&self, in_ray: &Ray, hit_data: &HitData) -> Option<(Ray, Color)>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, _in_ray: &Ray, hit_data: &HitData) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_data.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_data.normal;
        }
        Some((
            Ray {
                origin: hit_data.point,
                direction: scatter_direction,
            },
            self.albedo,
        ))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, in_ray: &Ray, hit_data: &HitData) -> Option<(Ray, Color)> {
        let reflected = Vec3::reflected_direction(&hit_data.normal, &in_ray.direction);
        let fuzz = clamp(self.fuzz, 0., 1.);
        let reflected = reflected.unit() + (Vec3::random_unit_vector() * fuzz);
        let scattered = Ray {
            origin: hit_data.point,
            direction: reflected,
        };
        if scattered.direction * hit_data.normal < 0. {
            return None;
        }
        Some((scattered, self.albedo))
    }
}

pub struct Dielectric {
    pub refraction_index: f64,
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, in_ray: &Ray, hit_data: &HitData) -> Option<(Ray, Color)> {
        let mut ri = self.refraction_index;
        if hit_data.front_face {
            ri = 1.0 / self.refraction_index;
        }

        let mut cos = hit_data.normal * in_ray.direction.unit();
        if cos > 1. {
            cos = 1.;
        }
        let sin = (1. - cos.powf(2.)).sqrt();
        if ri * sin > 1. || schlick(cos, ri) > rand::random_range(0. ..1.) {
            let reflected = Vec3::reflected_direction(&hit_data.normal, &in_ray.direction);
            Some((
                Ray {
                    origin: hit_data.point,
                    direction: reflected,
                },
                Vec3(1., 1., 1.),
            ))
        } else {
            let refracted = Vec3::refract(&in_ray.direction, &hit_data.normal, ri);
            Some((
                Ray {
                    origin: hit_data.point,
                    direction: refracted,
                },
                Vec3(1., 1., 1.),
            ))
        }
    }
}

pub fn hollow_sphere(
    refraction_index: f64,
    radius: f64,
    position: Vec3,
    thickness: f64,
) -> (Sphere, Sphere) {
    let outer_sphere = Sphere {
        center: position,
        radius,
        material: Arc::new(Dielectric { refraction_index }),
    };
    let inner_sphere = Sphere {
        center: position,
        radius: radius - thickness,
        material: Arc::new(Dielectric {
            refraction_index: 1. / refraction_index,
        }),
    };

    (outer_sphere, inner_sphere)
}
