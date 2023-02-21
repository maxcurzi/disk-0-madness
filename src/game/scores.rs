use crate::wasm4;

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
    /// bombs exploded in the current frame. Increase the multiplier every time
    /// something happens. Bombs are worth a lot more.
    pub fn update(&mut self, enemies_killed: u32, bombs_exploded: u32) {
        for _ in 0..bombs_exploded {
            self.current = self.current.wrapping_add(self.multiplier.wrapping_mul(10));
            self.multiplier = self.multiplier.wrapping_add(10);
        }
        for _ in 0..enemies_killed {
            self.current = self.current.wrapping_add(self.multiplier);
            self.multiplier = self.multiplier.wrapping_add(1);
        }
        self.current = self.current.clamp(0, 999_999_999);
    }
}
