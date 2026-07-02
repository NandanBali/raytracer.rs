use std::{fs::File, io::Write, ops};

use crate::helpers;

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn unit(&self) -> Vec3 {
        let sum = self.0.powf(2.) + self.1.powf(2.) + self.2.powf(2.);
        *self / sum.sqrt()
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
        255 * helpers::clamp(val, 0., 0.999) as u8
    }

    pub fn write_color(file: &mut File, color: Color) {
        let color_string = format!(
            "{} {} {}",
            Color::clmp(color.0),
            Color::clmp(color.1),
            Color::clmp(color.2)
        );
        let _ = writeln!(file, "{}", color_string);
    }
}
