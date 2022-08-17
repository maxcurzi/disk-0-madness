use std::f64::EPSILON;

use crate::entity::{Coord, Entity, Movable, Visible};
use crate::wasm4::SCREEN_SIZE;
use crate::{palette::set_draw_color, wasm4};

#[derive(Debug, Copy, Clone)]
pub struct Snake1(Entity);
pub struct Bomb(Entity);

impl Movable for Snake1 {
    fn update_position(&mut self) {
        self.0.update_position();
    }
}

impl Visible for Snake1 {
    // fn draw(&self) {
    //     set_draw_color(0x22);
    //     wasm4::oval(
    //         self.0.position.x as i32,
    //         self.0.position.y as i32,
    //         self.0.size as u32,
    //         self.0.size as u32,
    //     );
    //     // wasm4::oval(10, 10, 10, 10);
    // }

    // fn collided_with(&self, other: &Entity) -> bool {
    //     // Check if we're outside the square bounding box,
    //     !(((self.0.position.x > other.position.x + other.size as f64)
    //         || ((self.0.position.x + self.0.size as f64) < other.position.x))
    //         && ((self.0.position.y > other.position.y + other.size as f64)
    //             || ((self.0.position.y + self.0.size as f64) < other.position.y)))
    // }
    fn draw(&self) {
        set_draw_color(0x32);
        self.0.draw();
    }
    fn collided_with(&self, other: &Entity) -> bool {
        self.0.collided_with(other)
    }
}

impl Snake1 {
    pub fn new() -> Self {
        let mut s = Snake1(Entity::new());
        s.0.size = 6.0;
        s.0.position.x = 75.0;
        s.0.position.y = 75.0;
        s.0.direction.x = 1.0;
        s.0.speed = 1.35;
        s
    }

    pub fn left(&mut self) {
        self.0.direction = Coord {
            x: -1.0,
            y: self.0.direction.y,
        };
    }

    pub fn right(&mut self) {
        self.0.direction = Coord {
            x: 1.0,
            y: self.0.direction.y,
        };
    }

    pub fn up(&mut self) {
        self.0.direction = Coord {
            x: self.0.direction.x,
            y: -1.0,
        };
    }

    pub fn down(&mut self) {
        self.0.direction = Coord {
            x: self.0.direction.x,
            y: 1.0,
        };
    }
    pub fn stop(&mut self) {
        self.0.direction = Coord { x: 0.0, y: 0.0 };
    }

    pub fn get_position(&self) -> Coord {
        self.0.position
    }
    pub fn get_size(&self) -> u32 {
        self.0.size as u32
    }

    pub fn grow(&mut self) {
        let grow_amt = 0.8;
        self.0.size = (self.0.size + grow_amt).clamp(6.0, SCREEN_SIZE as f64);
        self.0.position.x = (self.0.position.x - (grow_amt as f64) / 2.0)
            .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
        self.0.position.y = (self.0.position.y - (grow_amt as f64) / 2.0)
            .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
    }
    pub fn shrink(&mut self) {
        let shrink_amt = 3.0;
        let shrinkage = self.0.size - (self.0.size - shrink_amt).clamp(6.0, SCREEN_SIZE as f64);
        self.0.size -= shrinkage;
        self.0.position.x = (self.0.position.x + (shrinkage as f64) / 2.0)
            .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
        self.0.position.y = (self.0.position.y + (shrinkage as f64) / 2.0)
            .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Bomb {
    pub fn new(pos: &Coord, size: f64) -> Self {
        let mut b = Bomb(Entity::new());
        b.0.size = size;
        b.0.position.x = pos.x;
        b.0.position.y = pos.y;
        b.0.speed = 0.0;
        b
    }

    pub fn get_position(&self) -> Coord {
        self.0.position
    }
    pub fn get_size(&self) -> u32 {
        self.0.size as u32
    }

    pub fn grow(&mut self) {
        let grow_amt = 0.8;
        self.0.size = (self.0.size + grow_amt).clamp(6.0, SCREEN_SIZE as f64);
        self.0.position.x = (self.0.position.x - (grow_amt as f64) / 2.0)
            .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
        self.0.position.y = (self.0.position.y - (grow_amt as f64) / 2.0)
            .clamp(0.0, (SCREEN_SIZE - self.get_size()) as f64);
    }

    pub fn draw_with_color(&self, color: u16) {
        set_draw_color(color);
        self.0.draw();
    }
}

impl Visible for Bomb {
    fn draw(&self) {
        self.0.draw();
    }
    fn collided_with(&self, other: &Entity) -> bool {
        self.0.collided_with(other)
    }
}
