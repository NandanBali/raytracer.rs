use crate::{ray::Ray, vec3::Vec3};

pub struct HitData {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitData {
    pub fn set_facenormal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = (ray.direction * *outward_normal) < 0.;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * -1.
        };
    }
}

pub trait Object {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitData>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Object for Sphere {
    fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitData> {
        let oc = self.center - ray.origin;
        let a = ray.direction * ray.direction;
        let b = oc * ray.direction;
        let c = oc * oc - self.radius.powf(2.);

        let discr = b.powf(2.) - a * c;
        if discr < 0. {
            return None;
        }

        let discr_sqrt = discr.sqrt();
        let mut proot = (b - discr_sqrt) / a;

        if proot < interval.0 || proot > interval.1 {
            let root = (b + discr_sqrt) / a;
            if root < interval.0 || root > interval.1 {
                return None;
            }
            proot = root;
        }

        let point = ray.at(proot);
        let normal = (point - self.center) / self.radius;

        let mut res = HitData {
            point,
            normal,
            t: proot,
            front_face: false,
        };

        res.set_facenormal(ray, &normal);
        Some(res)
    }
}

pub struct World {
    pub objects: Vec<Box<dyn Object>>,
}

impl World {
    pub fn new() -> Self {
        World { objects: vec![] }
    }

    pub fn add_obj(&mut self, obj: Box<dyn Object>) {
        self.objects.push(obj);
    }

    pub fn hit(&self, ray: &Ray, interval: (f64, f64)) -> Option<HitData> {
        let mut closest = interval.1;
        let mut result: Option<HitData> = None;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, (interval.0, closest)) {
                closest = hit.t;
                result = Some(hit);
            }
        }

        result
    }
}
