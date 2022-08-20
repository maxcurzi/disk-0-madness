use crate::wasm4::{DRAW_COLORS, PALETTE};

pub fn set_draw_color(idx: u16) {
    unsafe { *DRAW_COLORS = idx }
}

pub fn set_palette(palette: [u32; 4]) {
    unsafe {
        *PALETTE = palette;
    }
}

pub fn set_palette_n(palette_n: usize) {
    set_palette(PALETTES[palette_n]);
}

// Color info
#[rustfmt::skip]
pub const PALETTES: [[u32; 4]; 10] = [
    [0x120136, 0x035AA6, 0x40BAD5, 0xFCBF1E],
    [0x100720, 0x31087B, 0xFA2FB5, 0xFFC23C],
    [0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58],

    // https://lospec.com/palette-list/kirokaze-gameboy
    [0x332c50, 0x46878f, 0x94e344, 0xe2f3e4],

    // https://lospec.com/palette-list/red-blood-pain
    [0x7e1f23, 0xc4181f, 0x120a19, 0x5e4069],

    // https://lospec.com/palette-list/lava-gb
    [0x051f39, 0x4a2480, 0xc53a9d, 0xff8e80],

    // https://lospec.com/palette-list/game-watch-gb
    [0x06160f, 0x535b4e, 0xb0b3a6, 0xefeee8],

    // https://colorhunt.co/palette/001e6c035397e8630afcd900
    [0x001E6C, 0x035397, 0xE8630A, 0xFCD900],

    // https://colorhunt.co/palette/06113cff8c32ddddddeeeeee
    [0x06113C, 0xFF8C32, 0xDDDDDD, 0xEEEEEE],

    [0x12000A, 0x3B9E0C, 0x0A7E48, 0x9E0C50],
];

pub const COLOR1: u16 = 0x32;
pub const COLOR2: u16 = 0x34;
pub const COLOR_BOMB: u16 = 0x30;

#[rustfmt::skip]
pub const HEART: [u8; 8] = [
    0b00000000,
    0b00110110,
    0b01110111,
    0b01111111,
    0b01111111,
    0b00111110,
    0b00011100,
    0b00001000,
];
