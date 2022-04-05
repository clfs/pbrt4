use std::ops;

#[derive(Default)]
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl RGB {
    pub fn new(r: f64, g: f64, b: f64) -> RGB {
        debug_assert!(!r.is_nan() && !g.is_nan() && !b.is_nan());
        RGB { r, g, b }
    }

    fn has_nans(&self) -> bool {
        self.r.is_nan() || self.g.is_nan() || self.b.is_nan()
    }
}

impl ops::Add for RGB {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        debug_assert!(!rhs.has_nans());
        Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl ops::AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nans());
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl ops::Sub for RGB {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        debug_assert!(!rhs.has_nans());
        Self {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl ops::SubAssign for RGB {
    fn sub_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nans());
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl ops::Mul for RGB {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        debug_assert!(!rhs.has_nans());
        Self {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl ops::Mul<f64> for RGB {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        debug_assert!(!rhs.is_nan());
        Self {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl ops::MulAssign for RGB {
    fn mul_assign(&mut self, rhs: Self) {
        debug_assert!(!rhs.has_nans());
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl ops::MulAssign<f64> for RGB {
    fn mul_assign(&mut self, rhs: f64) {
        debug_assert!(!rhs.is_nan());
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}