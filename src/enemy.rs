use std::f64::EPSILON;

use crate::entity::{Coord, Entity, Movable, Visible};
use crate::snake::Snake1;

pub struct Enemy1(Entity);
impl Movable for Enemy1 {
    fn update_position(&mut self) {
        self.0.update_position();
        self.0.life -= 1;
    }
}

impl Enemy1 {
    pub fn follow(&mut self, snake: &Snake1) {
        let new_dir_x = snake.get_position().x + (snake.get_size() as f64 / 2.0)
            - (self.0.position.x + self.0.size as f64 / 2.0);
        let new_dir_y = snake.get_position().y + (snake.get_size() as f64 / 2.0)
            - (self.0.position.y + self.0.size as f64 / 2.0);
        let ddx = self.0.direction.x - new_dir_x;
        let ddy = self.0.direction.y - new_dir_y;
        let norm = (ddx * ddx + ddy * ddy).sqrt();
        if norm <= 2.0 * EPSILON {
            return;
        }
        let ddx_norm = (self.0.direction.x - new_dir_x / norm).clamp(-0.09, 0.09);
        let ddy_norm = (self.0.direction.y - new_dir_y / norm).clamp(-0.09, 0.09);
        self.0.direction.x -= ddx_norm;
        self.0.direction.y -= ddy_norm;
    }

    pub fn new(id: usize, x: f64, y: f64, color: u16) -> Self {
        let mut e = Enemy1(Entity::new());
        e.0.size = 4.0;
        e.0.position.x = x;
        e.0.position.y = y;
        e.0.direction.x = 0.0;
        e.0.speed = 0.7;
        e.0.id = id;
        e.0.color = color;
        e.0.life = 10 * 60; // n seconds at 60 FPS
        e
    }
    pub fn id(&self) -> usize {
        self.0.id
    }
    pub fn life(&self) -> i32 {
        self.0.life
    }
    pub fn get_position(&self) -> Coord {
        self.0.position
    }
    pub fn get_size(&self) -> f64 {
        self.0.size
    }
    pub fn kill(&mut self) {
        self.0.life = 0;
    }
    pub fn set_color(&mut self, color: u16) {
        self.0.color = color;
    }
    pub fn get_color(&self) -> u16 {
        self.0.color
    }
    pub fn collided_with(&self, snake: &Snake1) -> bool {
        let other: Entity = Entity {
            position: snake.get_position(),
            direction: Coord { x: 0.0, y: 0.0 }, //don't care
            size: snake.get_size() as f64,
            speed: 0.0, //don't care
            id: 0,      //don't care
            color: snake.get_color(),
            life: 0, //don't care
        };
        self.0.collided_with(&other)
    }
}

impl Visible for Enemy1 {
    fn draw(&self) {
        self.0.draw();
    }
    fn collided_with(&self, other: &Entity) -> bool {
        self.0.collided_with(other)
    }
}
