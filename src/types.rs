// Some useful types

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub const WHITE: Self = Self(255, 255, 255, 255);
    pub const BLACK: Self = Self(0, 0, 0, 255);
    pub const RED: Self = Self(255, 0, 0, 255);
    pub const GREEN: Self = Self(0, 255, 0, 255);
    pub const BLUE: Self = Self(0, 0, 255, 255);
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

#[derive(Clone, Debug, PartialEq)]
pub struct Vector2(pub f32, pub f32);

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

#[derive(Debug, Clone, PartialEq)]
pub struct Vector3(pub f32, pub f32, pub f32);

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

#[derive(Debug, Clone, PartialEq)]
pub struct Vector4(pub f32, pub f32, pub f32, pub f32);

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
