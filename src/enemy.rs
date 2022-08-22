use crate::entity::{Coord, Entity, Movable, Visible};
use crate::player::Player;

pub struct Enemy(Entity);

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
        let p_radius = player.get_size() / 2.0;
        let e_radius = self.get_size() / 2.0;

        let p_center = player.get_position()
            + Coord {
                x: p_radius,
                y: p_radius,
            };
        let e_center = self.get_position()
            + Coord {
                x: e_radius,
                y: e_radius,
            };
        let p_to_e = p_center - e_center;
        let norm = p_to_e.norm();
        if norm <= 2.0 * f64::EPSILON {
            return;
        }

        // Rate limit turns to make enemies slightly slower to follow sharp turns for more satisfying scapes
        let ddx_norm = (self.0.direction.x - p_to_e.x / norm).clamp(-0.09, 0.09);
        let ddy_norm = (self.0.direction.y - p_to_e.y / norm).clamp(-0.09, 0.09);
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
    pub fn get_id(&self) -> usize {
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

    pub fn just_spawned(&self) -> bool {
        self.life() > Self::DEFAULT_LIFE_SPAN - Self::I_FRAMES_ON_SPAWN
    }

    pub fn get_super(&self) -> &Entity {
        &self.0
    }
}

impl Movable for Enemy {
    fn update_position(&mut self) {
        self.0.update_position();
        self.0.life -= 1;
    }
}

impl Visible for Enemy {
    fn draw(&self) {
        self.0.draw();
    }
    // fn collided_with(&self, other: &impl Visible, extra_reach: f64) -> bool {
    //     self.0.collided_with(other, extra_reach)
    // }
    fn get_size(&self) -> f64 {
        self.0.size
    }
    fn get_position(&self) -> Coord {
        self.0.position
    }
}
