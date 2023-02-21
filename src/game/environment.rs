use crate::{
    common::calibrations::MUSIC_SPEED_CTRL,
    graphics::{draw_utils, palette},
    sound::{
        effects,
        music::{self, INTRO_SONG},
    },
    wasm4::SCREEN_SIZE,
};
use fastrand::Rng;

/// The Environment is responsible for drawing the play area (space), adjust
/// palette colours, and play music and sound effects. The HUD (score,
/// lives, etc..) is not part of the enviroment.
pub struct Environment {
    pub space: Vec<(u8, u8)>,
    pub palette_n: u8,
    pub song_nr: u8,
}
impl Environment {
    pub fn new(rng: &Rng) -> Self {
        const SPACE_PIXELS: u8 = 200;
        let mut space = vec![];
        for _ in 0..SPACE_PIXELS {
            space.push((
                rng.u8(0..(SCREEN_SIZE as u8)),
                rng.u8(0..(SCREEN_SIZE as u8)),
            ));
        }
        Self {
            space,
            palette_n: 0,
            song_nr: INTRO_SONG,
        }
    }

    pub fn draw_space(&self) {
        palette::set_draw_color(0x44);
        for p in &self.space {
            draw_utils::pixel(p.0 as i32, p.1 as i32);
        }
    }

    pub fn update(&self, _tick: usize, song_tick: usize) {
        music::play(song_tick / MUSIC_SPEED_CTRL, self.song_nr);
        self.draw_space();
    }

    pub fn play_sound_effects(&self, bombs_exploded: bool, extra_life: bool, player_died: bool) {
        // We just have very few sound effects.
        if bombs_exploded {
            effects::bomb_explode();
        }
        if extra_life {
            effects::extra_life();
        }
        if player_died {
            effects::death();
        }
    }

    pub fn set_palette(&mut self, palette_nr: u8) {
        self.palette_n = palette_nr % palette::PALETTES.len() as u8;
        palette::set_palette_n(self.palette_n as usize)
    }
}
