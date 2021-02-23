use oorandom::Rand32;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vector2(pub f32, pub f32);

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Vector2 {
        Vector2(x, y)
    }

    pub fn norm(self) -> f32 {
        self.norm_squared().sqrt()
    }

    pub fn norm_squared(self) -> f32 {
        self.0.powi(2) + self.1.powi(2)
    }

    pub fn world_to_screen_coords(self, screen_w_h: (f32, f32)) -> Vector2 {
        let x = self.0 + screen_w_h.0 / 2.0;
        let y = screen_w_h.1 - (self.1 + screen_w_h.1 / 2.0);
        Vector2(x, y)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1);
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1);
    }
}

impl Mul for Vector2 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Mul<f32> for Vector2 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Div for Vector2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        Self(self.0 / rhs.0, self.1 / rhs.1)
    }
}

impl Div<f32> for Vector2 {
    // The multiplication of rational numbers is a closed operation.
    type Output = Self;

    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs)
    }
}

pub fn random_vec(rng: &mut Rand32, max_magnitude: f32) -> Vector2 {
    let angle = rng.rand_float() * 2.0 * std::f32::consts::PI;
    let mag = rng.rand_float() * max_magnitude;
    vec_from_angle(angle) * mag
}

pub fn vec_from_angle(angle: f32) -> Vector2 {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2(vx, vy)
}
