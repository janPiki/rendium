// Some useful types

use std::ops::{Add, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub const WHITE: Self = Self(255, 255, 255, 255);
    pub const BLACK: Self = Self(0, 0, 0, 255);
    pub const RED: Self = Self(255, 0, 0, 255);
    pub const GREEN: Self = Self(0, 255, 0, 255);
    pub const BLUE: Self = Self(0, 0, 255, 255);

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(r, g, b, a)
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b, 255)
    }

    pub fn to_linear(self) -> [f32; 4] {
        [
            (self.0 as f32 / 255.0).powf(2.2),
            (self.1 as f32 / 255.0).powf(2.2),
            (self.2 as f32 / 255.0).powf(2.2),
            self.3 as f32 / 255.0,
        ]
    }
}

impl From<Color> for wgpu::Color {
    fn from(c: Color) -> Self {
        wgpu::Color {
            r: c.0 as f64 / 255.0,
            g: c.1 as f64 / 255.0,
            b: c.2 as f64 / 255.0,
            a: c.3 as f64 / 255.0,
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [
            c.0 as f32 / 255.0,
            c.1 as f32 / 255.0,
            c.2 as f32 / 255.0,
            c.3 as f32 / 255.0,
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2(pub f32, pub f32);

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self(x, y)
    }

    pub fn zero() -> Self {
        Self(0.0, 0.0)
    }

    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len != 0.0 {
            Self(self.0 / len, self.1 / len)
        } else {
            self
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.0 * other.0 + self.1 * other.1
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self(
            self.0 + (other.0 - self.0) * t,
            self.1 + (other.1 - self.1) * t,
        )
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl From<Vector2> for [f32; 2] {
    fn from(v: Vector2) -> Self {
        [v.0, v.1]
    }
}

impl From<(f32, f32)> for Vector2 {
    fn from(v: (f32, f32)) -> Self {
        Self(v.0, v.1)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3(pub f32, pub f32, pub f32);

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(x, y, z)
    }

    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0)
    }

    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn normalized(self) -> Self {
        let len = self.length();
        if len != 0.0 {
            Self(self.0 / len, self.1 / len, self.2 / len)
        } else {
            self
        }
    }

    pub fn dot(self, other: Self) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(self, other: Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    pub fn lerp(self, other: Self, t: f32) -> Self {
        Self(
            self.0 + (other.0 - self.0) * t,
            self.1 + (other.1 - self.1) * t,
            self.2 + (other.2 - self.2) * t,
        )
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Mul for Vector3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl From<Vector3> for [f32; 3] {
    fn from(v: Vector3) -> Self {
        [v.0, v.1, v.2]
    }
}

impl From<(f32, f32, f32)> for Vector3 {
    fn from(v: (f32, f32, f32)) -> Self {
        Self(v.0, v.1, v.2)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector4(pub f32, pub f32, pub f32, pub f32);

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self(x, y, z, w)
    }

    pub fn zero() -> Self {
        Self(0.0, 0.0, 0.0, 0.0)
    }
}

impl Sub for Vector4 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl From<Vector4> for [f32; 4] {
    fn from(v: Vector4) -> Self {
        [v.0, v.1, v.2, v.3]
    }
}

impl From<(f32, f32, f32, f32)> for Vector4 {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        Self(v.0, v.1, v.2, v.3)
    }
}
