use std::ops;

#[derive(Debug, Default, PartialEq)]
pub struct Vector2i {
    pub x: i64,
    pub y: i64,
}

impl Vector2i {
    pub fn new(x: i64, y: i64) -> Vector2i {
        Vector2i { x, y }
    }
}

impl ops::Add for Vector2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::AddAssign for Vector2i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl ops::Sub for Vector2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::SubAssign for Vector2i {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl ops::Mul<f64> for Vector2i {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        debug_assert!(!rhs.is_nan());
        Self {
            x: (self.x as f64 * rhs) as i64,
            y: (self.y as f64 * rhs) as i64,
        }
    }
}

impl ops::Mul<Vector2i> for f64 {
    type Output = Vector2i;

    fn mul(self, rhs: Vector2i) -> Vector2i {
        debug_assert!(!self.is_nan());
        Vector2i {
            x: (rhs.x as f64 * self) as i64,
            y: (rhs.y as f64 * self) as i64,
        }
    }
}

impl ops::MulAssign<f64> for Vector2i {
    fn mul_assign(&mut self, rhs: f64) {
        debug_assert!(!rhs.is_nan());
        self.x = (self.x as f64 * rhs) as i64;
        self.y = (self.y as f64 * rhs) as i64;
    }
}

impl ops::Div<f64> for Vector2i {
    type Output = Self;

    fn div(self, rhs: f64) -> Self {
        debug_assert!(!rhs.is_nan());
        Self {
            x: (self.x as f64 / rhs) as i64,
            y: (self.y as f64 / rhs) as i64,
        }
    }
}

impl ops::DivAssign<f64> for Vector2i {
    fn div_assign(&mut self, rhs: f64) {
        debug_assert!(!rhs.is_nan());
        self.x = (self.x as f64 / rhs) as i64;
        self.y = (self.y as f64 / rhs) as i64;
    }
}

impl ops::Neg for Vector2i {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vector2i {
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    pub fn lerp(self, other: Self, t: f64) -> Self {
        (1. - t) * self + t * other
    }

    pub fn componentwise_min(self, other: Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
}

pub struct Vector2f {
    pub x: f64,
    pub y: f64,
}

pub struct Vector3i {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Default)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Point2i {
    pub x: i64,
    pub y: i64,
}

pub struct Point2f {
    pub x: f64,
    pub y: f64,
}

pub struct Point3i {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

pub struct Point3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Normal3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub struct Bounds2i {
    pub p_min: Point2i,
    pub p_max: Point2i,
}

pub struct Bounds2f {
    pub p_min: Point2f,
    pub p_max: Point2f,
}

pub struct Bounds3i {
    pub p_min: Point3i,
    pub p_max: Point3i,
}

pub struct Bounds3f {
    pub p_min: Point3f,
    pub p_max: Point3f,
}

pub struct OctohedralVector {
    #[allow(dead_code)]
    x: u16,
    #[allow(dead_code)]
    y: u16,
}

pub struct DirectionCone {
    pub w: Vector3f,
    pub cos_theta: f64,
}

#[derive(Default)]
pub struct Frame {
    pub x: Vector3f,
    pub y: Vector3f,
    pub z: Vector3f,
}