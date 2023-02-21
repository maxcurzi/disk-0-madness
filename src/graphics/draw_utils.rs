use super::palette::{self, HEART};
use crate::wasm4::{self, BLIT_1BPP, DRAW_COLORS, FRAMEBUFFER, SCREEN_SIZE};

pub fn pixel(x: i32, y: i32) {
    // The byte index into the framebuffer that contains (x, y)
    let idx = ((y * SCREEN_SIZE as i32 + x) as usize) >> 2;

    // Calculate the bits within the byte that corresponds to our position
    let shift = (x as u8 & 0b11) << 1;
    let mask = 0b11 << shift;

    unsafe {
        let palette_color: u8 = (*DRAW_COLORS & 0xf) as u8;
        if palette_color == 0 {
            // Transparent
            return;
        }
        let color = (palette_color - 1) & 0b11;

        let framebuffer = &mut *FRAMEBUFFER;

        framebuffer[idx] = (color << shift) | (framebuffer[idx] & !mask);
    }
}

pub fn draw_hud(lives: u32, score: u32, high_score: u32, multiplier: u32, show_high_score: bool) {
    // Draws score, high-score, multiplier, player life count
    palette::set_draw_color(0x12);
    wasm4::text(score.to_string(), 1, 1);
    // if self.flags.new_high_score
    //     && self.flags.current_screen == ScreenName::GameOver
    //     && (self.timers.frame_count / 2) % 10 < 5
    // {
    //     palette::set_draw_color(0x00);
    // }
    if show_high_score {
        wasm4::text("H:".to_string() + high_score.to_string().as_str(), 73, 1);
    }
    palette::set_draw_color(0x12);
    wasm4::text(
        "x".to_string() + multiplier.to_string().as_str(),
        1,
        SCREEN_SIZE as i32 - 8,
    );
    // #[cfg(debug_assertions)]
    // text(
    //     "LVL:".to_string() + (self.calibrations.difficulty + 1).to_string().as_str(),
    //     60,
    //     SCREEN_SIZE as i32 - 8,
    // );
    palette::set_draw_color(0x20);
    let h_start: i32 = SCREEN_SIZE as i32 - 9;

    // Only Player 1 lives count, it owns the whole pool
    for l in 0..lives {
        wasm4::blit(
            &HEART,
            h_start - l as i32 * 8,
            SCREEN_SIZE as i32 - 9,
            8,
            8,
            BLIT_1BPP,
        );
    }
}
