#[cfg(feature = "buddy-alloc")]
mod alloc;
mod bomb;
mod draws;
mod enemy;
mod entity;
mod game;
mod palette;
mod player;
mod wasm4;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod common;
mod intro_screen;
mod notes;
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
