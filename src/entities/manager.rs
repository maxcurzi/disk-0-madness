use std::collections::HashMap;

use crate::{
    common::calibrations::{INIT_LIVES, MAX_BOMBS, MAX_ENEMIES},
    wasm4::SCREEN_SIZE,
};

use super::{
    bomb::Bomb,
    enemy::Enemy,
    player::{Player, PlayerN},
    traits::{Movable, Visible},
};

/// Entities include the player, enemies and bombs
pub struct EntityManager {
    pub players: [Option<Player>; 4],
    pub life_counter: u32,
    pub bombs: HashMap<usize, Box<Bomb>>,
    pub enemies: HashMap<usize, Box<Enemy>>,
    pub killer: Option<Enemy>,
}
impl EntityManager {
    pub fn new() -> Self {
        Self {
            players: [Some(Player::new(PlayerN::P1)), None, None, None],
            life_counter: INIT_LIVES,
            bombs: HashMap::with_capacity(MAX_BOMBS),
            enemies: HashMap::with_capacity(MAX_ENEMIES),
            killer: None,
        }
    }
    pub fn update(&mut self) -> (u32, u32) {
        self.update_state();

        let (enemies_killed, bombs_exploded) = self.process_collisions();
        (enemies_killed, bombs_exploded)
    }

    fn update_state(&mut self) {
        // Update players position
        for player in self.players.iter_mut().flatten() {
            player.update_position();
            player.stop();
        }

        // Update enemy position
        for enemy in self.enemies.values_mut() {
            let mut to_follow = &self.players[PlayerN::P1 as usize];

            let mut temp_distance = (SCREEN_SIZE * 2) as f64;
            for player in self.players.iter() {
                if let Some(p) = player {
                    let distance = enemy.entity.distance(&p.entity);
                    if distance < temp_distance {
                        temp_distance = distance;
                        to_follow = player;
                    };
                }
            }
            enemy.follow(to_follow);
            enemy.update_position();
        }

        // Update bombs
        for bomb in self.bombs.values_mut() {
            bomb.update();
        }

        // Cleanup dead entities
        self.prune();
    }

    fn prune(&mut self) {
        self.enemies.retain(|_, enemy| enemy.entity.life > 0);
        self.bombs.retain(|_, bomb| bomb.entity.life > 0);
    }

    pub fn draw(&self) {
        // Draw players before enemies so in case they overlap it would look more
        // "squishy" when escaping. Bombs should cover enemies so are drawn last.
        for p in self.players.iter().flatten() {
            p.draw();
        }
        for e in self.enemies.values() {
            e.draw();
        }
        for b in self.bombs.values() {
            b.draw();
        }
    }

    fn process_collisions(&mut self) -> (u32, u32) {
        // Returned variables
        let mut enemies_killed = 0;
        let mut bombs_exploded = 0;

        // Player-Bomb collision
        'bombs_loop: for bomb in self.bombs.values_mut() {
            for player in self.players.iter().flatten() {
                let extra_reach = 2.0; // Makes bombs easier to trigger
                if bomb.entity.collided_with(&player.entity, extra_reach) && !bomb.exploded {
                    bombs_exploded += 1;
                    bomb.exploded = true;
                    bomb.who_exploded = Some(player.player_number);
                    continue 'bombs_loop;
                }
            }
        }

        'enemies_loop: for enemy in self.enemies.values_mut() {
            // Bomb-Enemy collision, convert enemies to player color
            'bombs_loop: for bomb in self.bombs.values_mut() {
                let extra_reach = 2.0; // Makes enemies easier to convert
                if bomb.exploded && bomb.entity.collided_with(&enemy.entity, extra_reach) {
                    enemy.entity.color = self.players[bomb
                        .who_exploded
                        .expect("Exploded bomb should have a 'owner'")
                        as usize]
                        .as_ref()
                        .expect("Player should still be alive")
                        .entity
                        .color;
                    break 'bombs_loop; // One exploded bomb <=> One player
                }
            }

            // Enemy-Player collision (same color)
            'players_loop: for player in self.players.iter().flatten() {
                if enemy.entity.color == player.entity.color
                    && enemy.entity.collided_with(&player.entity, 2.0)
                {
                    enemy.kill();
                    enemies_killed += 1;
                    break 'players_loop; // Only one player should be able to "eat" one enemy
                }
            }

            // Enemy-Player collision (different colors)
            #[allow(unused_labels)]
            'players_loop: for player in self.players.iter().flatten() {
                if enemy.entity.color != player.entity.color
                    && !enemy.just_spawned()
                    && enemy.entity.collided_with(&player.entity, -2.0)
                {
                    // Player dies
                    self.killer = Some(Enemy::new(0, enemy.entity.position, enemy.entity.color));
                    enemy.kill();
                    break 'enemies_loop; // Stop everything.
                }
            }
        }

        self.prune();

        (enemies_killed, bombs_exploded)
    }
}
