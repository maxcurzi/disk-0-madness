use crate::wasm4;

pub fn set_draw_color(idx: u16) {
    unsafe { *wasm4::DRAW_COLORS = idx }
}

pub fn set_palette(palette: [u32; 4]) {
    unsafe {
        *wasm4::PALETTE = palette;
    }
}

// Color info
pub const PALETTES: [[u32; 4]; 4] = [
    [0x120136, 0x035AA6, 0x40BAD5, 0xFCBF1E],
    [0x100720, 0x31087B, 0xFA2FB5, 0xFFC23C],
    [0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58],
    [0x7777cc, 0x0044ff, 0xffcc00, 0xcc2200],
];

const BG_COLOR: u8 = 0;
const PLAYER_COLOR: u8 = 1;
const ENEMY_COLOR: u8 = 2;
const PWUP_COLOR: u8 = 3;
