use crate::palette::set_draw_color;
use crate::wasm4::{oval, SCREEN_SIZE};
use std::f64::EPSILON;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
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

impl Sub for Coord {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    pub position: Coord,
    pub direction: Coord,
    pub size: f64,
    pub speed: f64,
    pub id: usize,
    pub color: u16,
    pub life: i32,
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
}
// Trait for anything that can move on the screen
pub trait Movable {
    fn update_position(&mut self);
}
// Trait for anything that's visible on screen
pub trait Visible {
    fn draw(&self);
    fn collided_with(&self, other: &Entity) -> bool;
}

impl Movable for Entity {
    fn update_position(&mut self) {
        let mut norm = (self.direction.x.powi(2) + self.direction.y.powi(2)).sqrt();
        if norm <= EPSILON {
            norm = 1.0;
        }
        self.position.x += self.direction.x * self.speed / norm;
        self.position.y += self.direction.y * self.speed / norm;
        self.position.x = self
            .position
            .x
            .clamp(0.0, (SCREEN_SIZE as f64 - self.size) as f64);
        self.position.y = self
            .position
            .y
            .clamp(0.0, (SCREEN_SIZE as f64 - self.size) as f64);
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

    fn collided_with(&self, other: &Entity) -> bool {
        // // Square bounding box collision
        // ((self.position.x + (self.size - 1) as f64 > other.position.x)
        //     && (self.position.x < other.position.x + (other.size - 1) as f64))
        //     && ((self.position.y + (self.size - 1) as f64 > other.position.y)
        //         && (self.position.y < other.position.y + (other.size - 1) as f64))

        //Circular bounding box collision
        let radius1 = other.size / 2.0;
        let radius2 = self.size / 2.0;
        let x1_center = other.position.x + radius1;
        let y1_center = other.position.y + radius1;
        let x2_center = self.position.x + radius2;
        let y2_center = self.position.y + radius2;
        let dx = x1_center - x2_center;
        let dy = y1_center - y2_center;
        let distance = (dx * dx + dy * dy).sqrt();

        let mut tolerance = 0.0;
        if other.color != self.color {
            // Makes it easy to sneak past enemies for satisfying escapes
            tolerance = -2.0;
        }
        distance < radius1 + radius2 + tolerance
    }
}
