use fastrand::Rng;

use crate::palette::DRAW_COLOR_B;

pub const RNG_SEED: u64 = 555;
pub const MAX_ENEMIES: usize = 50;
pub const MAX_BOMBS: usize = 16;
pub const INIT_LIVES: u32 = 3;
pub const INIT_DIFFICULTY: u32 = 9;
pub const BOMB_FRAME_FREQ: usize = 300;
pub const MUSIC_SPEED_CTRL: usize = 5;
pub const DIFFICULTY_LEVELS: usize = 10;
pub const NEXT_LIFE_SCORE: u32 = 100_000;
pub const RESPITE_DURATION: usize = 120;
pub const DEATH_COUNTDOWN_DURATION: usize = 90;
// Enemy color switch times
pub const ENEMY_FRAME: [usize; DIFFICULTY_LEVELS] = [120, 60, 30, 25, 15, 10, 8, 6, 4, 2];
// Enemy spawn times
pub const EN_COL_FRAME: [usize; DIFFICULTY_LEVELS] = [240, 180, 160, 120, 100, 80, 60, 60, 60, 60];
// Score to difficulty
pub const DIFF_MUL_PROGRESSION: [u32; DIFFICULTY_LEVELS - 1] =
    [12, 30, 80, 120, 240, 320, 450, 1000, 2000];

/// Calibrations impact the gameplay difficulty, randomness, when the player
/// gets extra lives, etc...
pub struct Calibrations {
    pub difficulty: u32,
    pub score_next_life: u32,
    pub rng: Rng,
    pub enemy_color: u16,
}
impl Calibrations {
    pub fn new(tick_for_extra_rng: usize) -> Self {
        Self {
            difficulty: INIT_DIFFICULTY,
            score_next_life: NEXT_LIFE_SCORE,
            rng: Rng::with_seed(RNG_SEED + tick_for_extra_rng as u64),
            enemy_color: DRAW_COLOR_B,
        }
    }
}
