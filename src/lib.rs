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
use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod intro_screen;
mod music;
mod notes;
mod start_screen;
mod title_image;

lazy_static! {
    static ref PLAYER_GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette_n(0);
}

#[no_mangle]
fn update() {
    PLAYER_GAME.lock().expect("game_state").update();
}
