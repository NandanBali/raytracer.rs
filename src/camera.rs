use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub image_height: i32,
    pub image_width: i32,
    center: Vec3,         // camera center
    starting_point: Vec3, // location of 0, 0
    delta_u: Vec3,
    delta_v: Vec3,
    fov: f64,
    defocus_angle: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    fn degree_to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    pub fn new(
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

        Camera {
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
        }
   }
    pub fn antialiasing(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.starting_point
            + (self.delta_u * (i as f64 + offset.0))
            + (self.delta_v * (j as f64 + offset.1));
        let origin = if self.defocus_angle <= 0. {
            self.center
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
        self.center + (self.defocus_disk_u * p.0) + (self.defocus_disk_v * p.1)
    }

    fn sample_square() -> Vec3 {
        Vec3(
            rand::random_range(-0.5..0.5),
            rand::random_range(-0.5..0.5),
            0.,
        )
    }
}
