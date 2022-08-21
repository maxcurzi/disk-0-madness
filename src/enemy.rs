use std::f64::EPSILON;

use crate::entity::{Coord, Entity, Movable, Visible};
use crate::player::Player;
use crate::wasm4::trace;

pub struct Enemy(Entity);
impl Movable for Enemy {
    fn update_position(&mut self) {
        self.0.update_position();
        self.0.life -= 1;
    }
}

impl Enemy {
    // Enemies souldn't live forever. This is to avoid that very skilled players
    // may be able to kite all enemies in a single bubble and prevent further
    // enemies to respawn once the max number of enemies are on screen.
    const DEFAULT_LIFE_SPAN: u32 = 10 * 60;

    // Grant immunity to player if the enemy just spawned. It avoid frustration
    // of dying when enemies spawn just as you glide by the edges.
    const I_FRAMES_ON_SPAWN: u32 = 12;

    const DEFAULT_SPEED: f64 = 0.7;
    const DEFAULT_SIZE: f64 = 5.0;

    pub fn follow(&mut self, player: &Player) {
        // Standard pure pursuit
        let new_dir_x = player.get_position().x + (player.get_size() as f64 / 2.0)
            - (self.0.position.x + self.0.size as f64 / 2.0);
        let new_dir_y = player.get_position().y + (player.get_size() as f64 / 2.0)
            - (self.0.position.y + self.0.size as f64 / 2.0);
        let ddx = self.0.direction.x - new_dir_x;
        let ddy = self.0.direction.y - new_dir_y;
        let norm = (ddx * ddx + ddy * ddy).sqrt();
        if norm <= 2.0 * EPSILON {
            return;
        }

        // Rate limit turns to make enemies slightly slower to follow sharp turns for more satisfying scapes
        let ddx_norm = (self.0.direction.x - new_dir_x / norm).clamp(-0.09, 0.09);
        let ddy_norm = (self.0.direction.y - new_dir_y / norm).clamp(-0.09, 0.09);
        self.0.direction.x -= ddx_norm;
        self.0.direction.y -= ddy_norm;
    }

    pub fn new(id: usize, x: f64, y: f64, color: u16) -> Self {
        let mut e = Enemy(Entity::new());
        e.0.size = Self::DEFAULT_SIZE;
        e.0.position.x = x;
        e.0.position.y = y;
        e.0.direction.x = 0.0;
        e.0.speed = Self::DEFAULT_SPEED;
        e.0.id = id;
        e.0.color = color;
        e.0.life = Self::DEFAULT_LIFE_SPAN; // n seconds at 60 FPS
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
        if self.0.life > (Self::DEFAULT_LIFE_SPAN - Self::I_FRAMES_ON_SPAWN)
            && player.get_color() != self.get_color()
        {
            return false;
        }
        let extra_reach: f64 = if player.get_color() != self.get_color() {
            -2.5 // Reduce player size to allow satisfying escapes
        } else {
            0.5 // Increase player size to make easy to absorbe
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
