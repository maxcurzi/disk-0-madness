use std::f64::EPSILON;

use crate::entity::{Coord, Entity, Movable, Visible};
use crate::player::Player;

const ENEMY_SIZE: f64 = 4.0;
const ENEMY_SPEED: f64 = 0.7;
const ENEMY_LIFE: u32 = 12 * 60; // n seconds at 60 FPS

// Rate limit turns to make enemies slightly slower to
// follow sharp turns for more satisfying escapes
const TURN_RATE_LIMIT: f64 = 0.09;

pub struct Enemy(Entity);
impl Movable for Enemy {
    fn update_position(&mut self) {
        self.0.update_position();
        self.0.life -= 1;
    }
}

impl Enemy {
    pub fn follow(&mut self, player: &Player) {
        // Standard pure pursuit logic
        let player_center_x = player.get_position().x + (player.get_size() as f64 / 2.0);
        let player_center_y = player.get_position().y + (player.get_size() as f64 / 2.0);
        let self_center_x = self.0.position.x + self.0.size as f64 / 2.0;
        let self_center_y = self.0.position.x + self.0.size as f64 / 2.0;
        let new_dir_x = player_center_x - self_center_x;
        let new_dir_y = player_center_y - self_center_y;

        // Limit turn rate
        let ddx = self.0.direction.x - new_dir_x;
        let ddy = self.0.direction.y - new_dir_y;
        let norm = (ddx * ddx + ddy * ddy).sqrt();
        if norm <= 2.0 * EPSILON {
            return;
        }

        let ddx_norm =
            (self.0.direction.x - new_dir_x / norm).clamp(-TURN_RATE_LIMIT, TURN_RATE_LIMIT);
        let ddy_norm =
            (self.0.direction.y - new_dir_y / norm).clamp(-TURN_RATE_LIMIT, TURN_RATE_LIMIT);
        self.0.direction.x -= ddx_norm;
        self.0.direction.y -= ddy_norm;
    }

    pub fn new(id: usize, x: f64, y: f64, color: u16) -> Self {
        let mut e = Enemy(Entity::new());
        e.0.size = ENEMY_SIZE;
        e.0.position = Coord { x, y };
        e.0.direction = Coord { x: 0.0, y: 0.0 };
        e.0.speed = ENEMY_SPEED;
        e.0.id = id;
        e.0.color = color;
        e.0.life = ENEMY_LIFE;
        e
    }
    pub fn id(&self) -> usize {
        self.0.id
    }
    pub fn life(&self) -> u32 {
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
    pub fn collided_with(&self, player: &Player) -> bool {
        let extra_reach: f64 = if player.get_color() != self.get_color() {
            -2.0 // Reduce player size to allow satisfying escapes
        } else {
            1.0 // Increase player size to make easy to absorbe
        };

        let other: Entity = Entity {
            position: Coord {
                x: player.get_position().x - extra_reach,
                y: player.get_position().y - extra_reach,
            },
            direction: Coord { x: 0.0, y: 0.0 }, //don't care
            size: player.get_size() as f64 + extra_reach * 2.0,
            speed: 0.0, //don't care
            id: 0,      //don't care
            color: player.get_color(),
            life: 0, //don't care
        };
        self.0.collided_with(&other)
    }
}

impl Visible for Enemy {
    fn draw(&self) {
        self.0.draw();
    }
    fn collided_with(&self, other: &Entity) -> bool {
        self.0.collided_with(other)
    }
}
