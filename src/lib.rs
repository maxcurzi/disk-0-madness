#[cfg(feature = "buddy-alloc")]
mod alloc;
mod entities;
mod game;
mod graphics;
mod wasm4;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod common;
mod sound;
lazy_static! {
    static ref PLAYER_GAME: Mutex<game::Game> = Mutex::new(game::Game::new());
}

#[no_mangle]
fn start() {
    graphics::palette::set_palette_n(0);
}

#[no_mangle]
fn update() {
    PLAYER_GAME.lock().expect("game_state").update();
}
