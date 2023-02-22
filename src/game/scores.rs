use crate::{
    common::calibrations::{SCORE_BOMB, SCORE_ENEMY},
    wasm4,
};

/// Score simply depends on enemies absorbed and bombs exploded. Each enemy/bomb
/// gives an increasing amount of score, defined by the multiplier.
pub struct Scores {
    pub current: u32,
    pub multiplier: u32,
    pub high: u32,
}
impl Scores {
    pub fn new() -> Self {
        Self {
            current: 0,
            multiplier: 1,
            high: Scores::get_high_score(),
        }
    }
    pub fn get_high_score() -> u32 {
        unsafe {
            let mut buffer = [0u8; core::mem::size_of::<u32>()];
            wasm4::diskr(buffer.as_mut_ptr(), buffer.len() as u32);
            u32::from_le_bytes(buffer)
        }
    }
    pub fn save_high_score(&self, high_score: u32) {
        unsafe {
            let high_score_bytes = high_score.to_le_bytes();
            wasm4::diskw(
                high_score_bytes.as_ptr(),
                core::mem::size_of::<u32>() as u32,
            );
        }
    }
    /// Updates the player's score depening on how many enemies were killed and
    /// bombs exploded in the current frame.
    pub fn update(&mut self, enemies_killed: u32, bombs_exploded: u32) {
        for _ in 0..bombs_exploded {
            self.current = self
                .current
                .wrapping_add(self.multiplier.wrapping_mul(SCORE_BOMB));
            self.multiplier = self.multiplier.wrapping_add(SCORE_BOMB);
        }
        for _ in 0..enemies_killed {
            self.current = self.current.wrapping_add(self.multiplier);
            self.multiplier = self.multiplier.wrapping_add(SCORE_ENEMY);
        }
        self.current = self.current.clamp(0, 999_999_999);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_update() {
        let mut scores = Scores {
            current: 0,
            multiplier: 1,
            high: 0,
        };
        let init_score = scores.current;
        let init_multiplier = scores.multiplier;
        scores.update(0, 0);
        assert_eq!(scores.current, 0);
        assert_eq!(scores.multiplier, init_multiplier);

        scores.update(1, 0);
        assert_eq!(scores.current, init_score + init_multiplier);
        assert_eq!(scores.multiplier, init_multiplier + SCORE_ENEMY);

        scores.update(0, 1);
        let mut exp_score =
            init_score + init_multiplier + (init_multiplier + SCORE_ENEMY) * SCORE_BOMB;
        assert_eq!(scores.current, exp_score);

        let mut exp_mult = init_multiplier + SCORE_ENEMY + SCORE_BOMB;
        assert_eq!(scores.multiplier, exp_mult);
        scores.update(1, 1);
        exp_score += (exp_mult * SCORE_BOMB) + (exp_mult + SCORE_BOMB);

        assert_eq!(scores.current, exp_score);
        exp_mult += SCORE_BOMB + SCORE_ENEMY;
        assert_eq!(scores.multiplier, exp_mult);
    }
}
