use crate::{
    common::{Coord, Movable, Visible},
    entity::Entity,
    palette::DRAW_COLOR_A,
    player::{Player, PlayerN},
};

#[derive(Clone)]
pub struct Enemy {
    pub entity: Entity,
    pub follows: PlayerN,
}

impl Enemy {
    // Enemies shouldn't live forever. This is to avoid that very skilled players
    // may be able to kite all enemies in a single bubble and prevent further
    // enemies to respawn once the max number of enemies are on screen.
    const LIFE_SPAN: u32 = 10 * 60;

    // Grant immunity to player if the enemy just spawned. It avoid frustration
    // of dying when enemies spawn just as you glide by the edges.
    const I_FRAMES_ON_SPAWN: u32 = 12;

    pub fn follow(&mut self, player: &Option<Player>) {
        if let Some(player) = player {
            self.follows = player.player_number;
            // Standard pure pursuit
            let p_radius = player.entity.size / 2.0;
            let e_radius = self.entity.size / 2.0;

            let p_center = player.entity.position
                + Coord {
                    x: p_radius,
                    y: p_radius,
                };
            let e_center = self.entity.position
                + Coord {
                    x: e_radius,
                    y: e_radius,
                };
            let p_to_e = p_center - e_center;
            let norm = p_to_e.norm();
            if norm <= 2.0 * f64::EPSILON {
                return;
            }

            // Rate limit turns to make enemies slightly slower to follow sharp turns for more satisfying escapes
            let ddx_norm = (self.entity.direction.x - p_to_e.x / norm).clamp(-0.09, 0.09);
            let ddy_norm = (self.entity.direction.y - p_to_e.y / norm).clamp(-0.09, 0.09);
            self.entity.direction.x -= ddx_norm;
            self.entity.direction.y -= ddy_norm;
        }
    }

    pub fn new(_id: usize, pos: Coord, color: u16) -> Self {
        let mut enemy = Self::default();
        enemy.entity.position = pos;
        enemy.entity.color = color;
        enemy
    }

    pub fn kill(&mut self) {
        self.entity.life = 0;
    }

    pub fn just_spawned(&self) -> bool {
        self.entity.life > Self::LIFE_SPAN - Self::I_FRAMES_ON_SPAWN
    }
}

impl Default for Enemy {
    fn default() -> Self {
        const DEFAULT_SPEED: f64 = 0.7;
        const DEFAULT_SIZE: f64 = 5.0;
        Self {
            entity: Entity {
                position: Coord::default(),
                direction: Coord::default(),
                size: DEFAULT_SIZE,
                speed: DEFAULT_SPEED,
                color: DRAW_COLOR_A,
                life: Self::LIFE_SPAN,
            },
            follows: PlayerN::P1,
        }
    }
}

impl Movable for Enemy {
    fn update_position(&mut self) {
        self.entity.update_position();
        self.entity.life -= 1;
    }
}

impl Visible for Enemy {
    fn draw(&self) {
        self.entity.draw();
    }
}
