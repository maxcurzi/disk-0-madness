use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}
impl Coord {
    /// Absolute distance between two points
    pub fn distance_to(self, other: &Self) -> f64 {
        let d_vect = self - *other;
        (d_vect.x.powi(2) + d_vect.y.powi(2)).sqrt()
    }

    /// Norm (length) of Coord vector
    pub fn norm(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    /// Scale Coordinate by a simple multiplication factor
    pub fn scale(self, scale: f64) -> Coord {
        Self {
            x: self.x * scale,
            y: self.y * scale,
        }
    }

    /// Clamps all coordinates between two values (x-y independent)
    pub fn clamp(self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Coord {
        Self {
            x: self.x.clamp(min_x, max_x),
            y: self.y.clamp(min_y, max_y),
        }
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_to() {
        let a = Coord { x: 0.0, y: 0.0 };
        let b = Coord { x: 3.0, y: 4.0 };
        assert_eq!(a.distance_to(&b), 5.0);
    }

    #[test]
    fn norm() {
        let a = Coord { x: 0.0, y: 0.0 };
        let b = Coord { x: 3.0, y: 4.0 };
        assert_eq!(a.norm(), 0.0);
        assert_eq!(b.norm(), 5.0);
    }

    #[test]
    fn scale() {
        let a = Coord { x: 0.0, y: 0.0 };
        let b = Coord { x: 3.0, y: 4.0 };
        assert_eq!(a.scale(2.0), a);
        assert_eq!(b.scale(2.0), Coord { x: 6.0, y: 8.0 });
    }

    #[test]
    fn clamp() {
        let a = Coord { x: 0.0, y: 0.0 };
        let b = Coord { x: 3.0, y: 4.0 };
        assert_eq!(a.clamp(0.0, 1.0, 0.0, 1.0), a);
        assert_eq!(b.clamp(0.0, 1.0, 0.0, 1.0), Coord { x: 1.0, y: 1.0 });
    }
}
