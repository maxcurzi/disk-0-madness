use crate::draws::pixel;
use crate::entity::{Coord, Entity, Movable, Visible};
use crate::snake::Snake1;
use crate::wasm4::SCREEN_SIZE;
use crate::{palette::set_draw_color, wasm4};
use std::f64::EPSILON;

// #[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
pub struct FPoint {
    pub x: f64,
    pub y: f64,
}

pub struct Enemy {
    pub pos: Point,
    pub direction: FPoint,
    pub _pos: FPoint,
    pub size: u32,
    pub speed: f64,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            pos: Point { x: 0, y: 0 },
            direction: FPoint { x: 0.0, y: 0.0 },
            _pos: FPoint { x: 0.0, y: 0.0 },
            size: 2,
            speed: 0.5,
        }
    }
    pub fn new_w_pos(x: i32, y: i32) -> Self {
        Self {
            pos: Point { x, y },
            direction: FPoint { x: 0.0, y: 0.0 },
            _pos: FPoint {
                x: x as f64,
                y: y as f64,
            },
            size: 2,
            speed: 0.5,
        }
    }

    pub fn draw(&self) {
        set_draw_color(0x33);

        wasm4::oval(self.pos.x, self.pos.y, self.size, self.size);
        // pixel(self.pos.x, self.pos.y);
        // set_draw_color(0x4);
    }

    pub fn update(&mut self) {
        let mut norm: f64 = (self.direction.x.powi(2) + self.direction.y.powi(2)).sqrt();
        if norm <= EPSILON {
            norm = 1.0;
        }
        self._pos.x += self.direction.x as f64 / norm * self.speed;
        self._pos.y += self.direction.y as f64 / norm * self.speed;
        self._pos.x = self._pos.x.clamp(0.0, 160.0 - self.size as f64);
        self._pos.y = self._pos.y.clamp(0.0, 160.0 - self.size as f64);
        self.pos.x = self._pos.x.round() as i32;
        self.pos.y = self._pos.y.round() as i32;
    }

    pub fn follow(&mut self, snake: &Snake1) {
        self.direction.x = snake.get_position().x + (snake.get_size() as f64 / 2.0)
            - (self._pos.x + self.size as f64 / 2.0);
        self.direction.y = snake.get_position().y + (snake.get_size() as f64 / 2.0)
            - (self._pos.y + self.size as f64 / 2.0);
    }

    pub fn stop(&mut self) {
        self.direction = FPoint { x: 0.0, y: 0.0 };
    }

    pub fn is_dead(&self) -> bool {
        // self.body
        //     .iter()
        //     .skip(1)
        //     .any(|&body_section| body_section == self.body[0])
        false
    }
}
pub struct Enemy1(Entity);
impl Movable for Enemy1 {
    fn update_position(&mut self) {
        // let rng = Rng::with_seed(42);
        // self.0.position.x = (self.0.position.x + (rng.f64() - 0.5) / 4.0)
        //     .clamp(0.0, SCREEN_SIZE as f64 - self.0.size as f64);
        // self.0.position.y = (self.0.position.y + (rng.f64() - 0.5) / 4.0)
        //     .clamp(0.0, SCREEN_SIZE as f64 - self.0.size as f64);
        self.0.update_position();
    }
}

impl Enemy1 {
    pub fn follow(&mut self, snake: &Snake1) {
        self.0.direction.x = snake.get_position().x + (snake.get_size() as f64 / 2.0)
            - (self.0.position.x + self.0.size as f64 / 2.0);
        self.0.direction.y = snake.get_position().y + (snake.get_size() as f64 / 2.0)
            - (self.0.position.y + self.0.size as f64 / 2.0);
    }

    pub fn new(id: u32, x: f64, y: f64) -> Self {
        let mut e = Enemy1(Entity::new());
        e.0.size = 4.0;
        e.0.position.x = x;
        e.0.position.y = y;
        e.0.direction.x = 0.0;
        e.0.speed = 0.7;
        e.0.id = id;
        e
    }
    pub fn id(&self) -> u32 {
        self.0.id
    }

    pub fn collided_with(&self, snake: &Snake1) -> bool {
        let other: Entity = Entity {
            position: snake.get_position(),
            direction: Coord { x: 0.0, y: 0.0 },
            size: snake.get_size() as f64,
            speed: 0.0,
            id: 0,
        };
        self.0.collided_with(&other)
    }
}

impl Visible for Enemy1 {
    fn draw(&self) {
        set_draw_color(0x34);
        self.0.draw();
    }
    fn collided_with(&self, other: &Entity) -> bool {
        self.0.collided_with(other)
    }
}
