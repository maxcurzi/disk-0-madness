use crate::wasm4::{DRAW_COLORS, FRAMEBUFFER};

pub fn pixel(x: i32, y: i32) {
    // The byte index into the framebuffer that contains (x, y)
    let idx = (y as usize * 160 + x as usize) >> 2;

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
