#[cfg(feature = "buddy-alloc")]
mod alloc;
mod draws;
mod enemy;
mod entity;
// mod entity::entity_tests;
mod game;
mod palette;
mod snake;
mod wasm4;
use game::Game;
use lazy_static::lazy_static;
use std::sync::Mutex;
mod collisions;
use palette::PALETTES;

lazy_static! {
    static ref SNAKE_GAME: Mutex<Game> = Mutex::new(Game::new());
}

#[no_mangle]
fn start() {
    // palette::set_palette([0xfff6d3, 0xf9a875, 0xeb6b6f, 0x7c3f58]);
    palette::set_palette(PALETTES[0]);
}

#[no_mangle]
fn update() {
    SNAKE_GAME.lock().expect("game_state").update();
}

const BG_COLOR: u8 = 0;
const SNAKE_COLOR: u8 = 1;
const ENEMY_COLOR: u8 = 2;
const PWUP_COLOR: u8 = 3;
