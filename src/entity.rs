use crate::palette::set_draw_color;
use crate::wasm4::{oval, SCREEN_SIZE};
use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
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

#[derive(Debug)]
pub struct Entity {
    pub position: Coord,
    pub direction: Coord,
    pub size: f64,
    pub speed: f64,
    pub id: usize,
    pub color: u16,
    pub life: u32,
}
impl Entity {
    pub fn new() -> Self {
        Self {
            position: Coord { x: 0.0, y: 0.0 },
            direction: Coord { x: 0.0, y: 0.0 },
            size: 1.0,
            speed: 0.0,
            id: 0,
            color: 0,
            life: 0,
        }
    }

    /// Distance between the center of two visible entities (assumed circular)
    pub fn distance(&self, other: &impl Visible) -> f64 {
        let radius1 = other.get_size() / 2.0 - 0.5;
        let radius2 = self.get_size() / 2.0 - 0.5;

        let center1 = other.get_position()
            + Coord {
                x: radius1,
                y: radius1,
            };
        let center2 = self.get_position()
            + Coord {
                x: radius2,
                y: radius2,
            };

        center1.distance_to(&center2)
    }

    pub fn collided_with(&self, other: &impl Visible, extra_reach: f64) -> bool {
        //Circular bounding box collision
        let radius1 = other.get_size() / 2.0 - 0.5;
        let radius2 = self.get_size() / 2.0 - 0.5;

        self.distance(other) < radius1 + radius2 + extra_reach
    }
}

// Trait for anything that can move on the screen
pub trait Movable {
    fn update_position(&mut self);
}
// Trait for anything that's visible on screen
pub trait Visible {
    fn draw(&self);
    fn get_size(&self) -> f64;
    fn get_position(&self) -> Coord;
    // fn collided_with(&self, other: &impl Visible, extra_reach: f64) -> bool;
}

impl Movable for Entity {
    fn update_position(&mut self) {
        let mut norm = self.direction.norm();
        if norm <= f64::EPSILON {
            return;
        }
        self.position += self.direction.scale(self.speed / norm);
        self.position = self.position.clamp(
            0.0,
            SCREEN_SIZE as f64 - self.size,
            0.0,
            SCREEN_SIZE as f64 - self.size,
        );
    }
}

impl Visible for Entity {
    fn draw(&self) {
        set_draw_color(self.color);
        oval(
            self.position.x as i32,
            self.position.y as i32,
            self.size as u32,
            self.size as u32,
        );
    }

    fn get_size(&self) -> f64 {
        self.size
    }
    fn get_position(&self) -> Coord {
        self.position
    }
}
