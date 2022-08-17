// use std::collections::hash_set::HashSet;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::draws::pixel;
use crate::enemy::{Enemy, Enemy1};
use crate::entity::{Coord, Entity};
use crate::entity::{Movable, Visible};
use crate::palette::set_draw_color;
use crate::snake::Bomb;
use crate::snake::{Point, Snake1};
use crate::wasm4;
use crate::wasm4::SCREEN_SIZE;
use fastrand::Rng;

const FRUIT_SPRITE: [u8; 16] = [
    0x00, 0xa0, 0x02, 0x00, 0x0e, 0xf0, 0x36, 0x5c, 0xd6, 0x57, 0xd5, 0x57, 0x35, 0x5c, 0x0f, 0xf0,
];
const MAX_ENEMIES: usize = 120;

pub struct Game {
    rng: Rng,
    frame_count: u32,
    snake: Snake1,
    prev_gamepad: u8,
    fruit: Point,
    enemies: Vec<Enemy>,
    enemies1: HashMap<u32, Box<Enemy1>>,
}
impl Game {
    pub fn new() -> Self {
        let rng = Rng::with_seed(235);

        Self {
            frame_count: 0,
            snake: Snake1::new(),
            prev_gamepad: 0,
            fruit: Point {
                x: rng.i32(0..160),
                y: rng.i32(0..160),
            },
            rng,
            enemies: vec![Enemy::new()],
            enemies1: HashMap::new(),
        }
    }

    pub fn input(&mut self) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed = gamepad; // & (gamepad ^ self.prev_gamepad);
        self.snake.stop();
        if just_pressed & wasm4::BUTTON_LEFT != 0 {
            self.snake.left();
        }
        if just_pressed & wasm4::BUTTON_RIGHT != 0 {
            self.snake.right();
        }
        if just_pressed & wasm4::BUTTON_UP != 0 {
            self.snake.up();
        }
        if just_pressed & wasm4::BUTTON_DOWN != 0 {
            self.snake.down();
        }
        if just_pressed & wasm4::BUTTON_2 != 0 {
            self.snake.down();
        }
        self.prev_gamepad = gamepad;
    }

    pub fn update(&mut self) {
        // static bomb: Bomb = Bomb::new(&Coord { x: 0.0, y: 0.0 }, 5.0);
        self.draw_space();
        self.frame_count += 1;

        self.input();

        if self.frame_count % 5 == 0 && self.enemies1.len() < MAX_ENEMIES {
            // self.enemies.push(Enemy::new_w_pos(
            //     self.rng.i32(0..=1) * SCREEN_SIZE as i32,
            //     self.rng.i32(0..=1) * SCREEN_SIZE as i32,
            // ));
            self.enemies1.insert(
                self.frame_count,
                Box::new(Enemy1::new(
                    self.frame_count,
                    (self.rng.i32(0..=2) * (SCREEN_SIZE / 2) as i32) as f64,
                    (self.rng.i32(0..=2) * (SCREEN_SIZE / 2) as i32) as f64,
                )),
            );
        }

        self.snake.update_position();

        // for enemy in self.enemies.iter_mut() {
        //     enemy.follow(&self.snake);
        //     enemy.update();
        // }
        for (_, enemy) in self.enemies1.iter_mut() {
            enemy.follow(&self.snake);
            // *enemy.follow(&self.snake);
            enemy.update_position();
        }

        self.snake.draw();
        // for enemy in &self.enemies {
        //     enemy.draw();
        // }
        let mut to_delete: Vec<u32> = vec![];
        for enemy in self.enemies1.values() {
            if enemy.collided_with(&self.snake) {
                to_delete.push(enemy.id());
                // self.snake.grow();
            } else {
                enemy.draw();
            }
        }
        // if self.frame_count % 60 == 0 {
        //     self.snake.shrink();
        // }
        for td in to_delete {
            self.enemies1.remove(&td).unwrap();
        }
        // set_draw_color(0x31);
        // wasm4::oval(-98, -98, 198, 198);
        // set_draw_color(0x21);
        // wasm4::oval(-100, -100, 200, 200);
        // set_draw_color(0x31);
        // wasm4::oval(-99, -99, 199, 199);
    }

    fn draw_space(&self) {
        let n_pixels = 1000;
        set_draw_color(0x44);
        for _ in 0..n_pixels {
            pixel(
                self.rng.i32(0..(SCREEN_SIZE as i32)),
                self.rng.i32(0..(SCREEN_SIZE as i32)),
            );
        }
    }
}
