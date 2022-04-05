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