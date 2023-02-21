#[cfg(feature = "buddy-alloc")]
mod alloc;
mod draw_utils;
mod entities;
mod game;
mod palette;
mod wasm4;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod calibrations;
mod common_types;
mod controls;
mod intro_screen;
mod screen;
mod sound;
mod title_image;

lazy_static! {
    static ref PLAYER_GAME: Mutex<game::Game> = Mutex::new(game::Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette_n(0);
}

#[no_mangle]
fn update() {
    PLAYER_GAME.lock().expect("game_state").update();
}
