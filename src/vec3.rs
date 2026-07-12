use std::{fs::File, io::Write, ops};
use crate::helpers;
use crate::ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn unit(&self) -> Vec3 {
        let sum = self.0.powf(2.) + self.1.powf(2.) + self.2.powf(2.);
        *self / sum.sqrt()
    }

    pub fn random() -> Vec3 {
        let (min, max) = (0., 1.);
        let x = rand::random_range(min..max);
        let y = rand::random_range(min..max);
        let z = rand::random_range(min..max);

        Vec3(x, y, z)
    }

    /// generates a ray that is 1) normalized 2) inside the sphere
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = (Vec3::random() * 2.) + -1.;
            let len = (p.0 * p.0) + (p.1 * p.1) + (p.2 * p.2);
            if 0.0000001 < len && len < 1. {
                return p / len.sqrt();
            }
        }
    }

    /// inverts the ray if it's pointing into the hemisphere
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let rn = Self::random_unit_vector();
        if rn * (*normal) > 0. {
            rn
        } else {
            rn * -1.
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.0.abs() < s && self.1.abs() < s && self.2.abs() < s
    }

    pub fn reflected_direction(normal: &Vec3, inbound: &Vec3) -> Vec3 {
        let v = inbound.clone();
        let n = normal.clone();

        v - n * (v * n * 2.)
    }
    
    pub fn stmul(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3 (v1.0 * v2.0, v1.1 * v2.1, v1.2 * v2.2)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f64) -> Self::Output {
        Vec3(self.0 + rhs, self.1 + rhs, self.2 + rhs)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl ops::Mul for Vec3 {
    type Output = f64;
    fn mul(self, rhs: Self) -> Self::Output {
        (self.0 * rhs.0) + (self.1 * rhs.1) + (self.2 * rhs.2)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

// Color

pub type Color = Vec3;

impl Color {
    fn clmp(val: f64) -> u8 {
        (255. * helpers::clamp(val, 0., 0.999)) as u8
    }

    fn l2g(lc: f64) -> f64 {
        if lc > 0. {
            return lc.sqrt();
        }
        0.
    }

    pub fn write_color(file: &mut File, color: Color) {
        let color_string = format!(
            "{} {} {}",
            Color::clmp(Self::l2g(color.0)),
            Color::clmp(Self::l2g(color.1)),
            Color::clmp(Self::l2g(color.2))
        );
        let _ = writeln!(file, "{}", color_string);
    }
}
