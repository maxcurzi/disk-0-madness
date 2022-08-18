#[cfg(feature = "buddy-alloc")]
mod alloc;
mod bomb;
mod draws;
mod enemy;
mod entity;
mod game;
mod palette;
mod snake;
mod wasm4;
use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod collisions;
mod start_screen;

lazy_static! {
    static ref SNAKE_GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    palette::set_palette_n(0);
}

#[no_mangle]
fn update() {
    SNAKE_GAME.lock().expect("game_state").update();
}
