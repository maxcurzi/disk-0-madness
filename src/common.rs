use std::ops::{Add, AddAssign, Sub};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

// Trait for anything that can move on the screen
pub trait Movable {
    fn update_position(&mut self);
}
// Trait for anything that's visible on screen
pub trait Visible {
    fn draw(&self);
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
