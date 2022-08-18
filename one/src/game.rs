use std::cmp::max;
use std::collections::HashMap;

use crate::bomb::Bomb;
use crate::draws::pixel;
use crate::enemy::Enemy1;
use crate::entity::Coord;
use crate::entity::{Movable, Visible};
use crate::palette::COLOR2;
use crate::palette::HEART;
use crate::palette::{self, set_draw_color};
use crate::palette::{COLOR1, PALETTES};
use crate::snake::Snake1;
use crate::start_screen::htp_screen;
use crate::wasm4::diskr;
use crate::wasm4::diskw;
use crate::wasm4::text;
use crate::wasm4::BLIT_1BPP;
use crate::wasm4::MOUSE_BUTTONS;
use crate::wasm4::MOUSE_LEFT;
use crate::wasm4::MOUSE_RIGHT;
use crate::wasm4::MOUSE_X;
use crate::wasm4::MOUSE_Y;
use crate::wasm4::SCREEN_SIZE;
use crate::wasm4::{self, MOUSE_MIDDLE};
use crate::wasm4::{blit, BUTTON_2};
use fastrand::Rng;

const MAX_ENEMIES: usize = 120;
const MAX_BOMBS: usize = 16;
const INIT_LIVES: i32 = 4;
const BOMB_FRAME_FREQ: u32 = 400;

pub struct Game {
    rng: Rng,
    frame_count: u32,
    snake: Snake1,
    prev_gamepad: u8,
    prev_mouse: u8,
    bombs: HashMap<u32, Box<(Bomb, bool)>>,
    enemies1: HashMap<u32, Box<Enemy1>>,
    space: Vec<(u8, u8)>,
    enemy_color: u16,
    en_frame: Vec<u8>,
    difficulty: u8,
    en_col_frame: Vec<u8>,
    score: u32,
    high_score: u32,
    multiplier: u32,
    diff_mul_spike: Vec<u32>,
    respite: i32,           // frames without enemies
    killer: Option<Enemy1>, // store enemy that killed player
    death_countdown: i32,
    score_next_life: u32,
    palette_n: u8,
    show_htp: bool,
    show_game_over: bool,
}

impl Game {
    pub fn initialize_game(&mut self) {
        self.difficulty = 0;
        self.score = 0;
        self.multiplier = 1;
        let mut snake = Snake1::new();
        snake.set_life(INIT_LIVES);
        self.snake = snake;
        self.enemies1.drain();
        self.bombs.drain();
        self.respite = 0;
        self.killer = None;
        self.death_countdown = 6;
        self.score_next_life = 100_000;
        self.palette_n = 0;
        self.show_htp = false;
        self.show_game_over = false;
    }
    pub fn new() -> Self {
        let rng = Rng::with_seed(555);
        let en_frame: Vec<u8> = vec![120, 60, 30, 25, 10, 8, 6, 4, 2, 1];
        let en_col_frame: Vec<u8> = vec![240, 180, 160, 120, 100, 80, 60, 60, 60, 60];
        let diff_mul_spike = vec![4, 20, 50, 100, 300, 600, 1000, 1500, 2000, 3000];
        let difficulty = 0;
        let score: u32 = 0;
        let respite = 0;
        let multiplier = 1;
        let killer: Option<Enemy1> = None;
        let death_countdown = 6;
        let score_next_life = 100_000;
        let palette_n = 0;
        let show_htp = true;
        let show_game_over = false;
        const SPACE_PIXELS: u8 = 200;
        let mut space = vec![];
        for _ in 0..SPACE_PIXELS {
            space.push((
                rng.u8(0..(SCREEN_SIZE as u8)),
                rng.u8(0..(SCREEN_SIZE as u8)),
            ));
        }
        let high_score = unsafe {
            let mut buffer = [0u8; core::mem::size_of::<u32>()];

            diskr(buffer.as_mut_ptr(), buffer.len() as u32);

            u32::from_le_bytes(buffer)
        };
        let mut snake = Snake1::new();
        snake.set_life(INIT_LIVES);

        Self {
            frame_count: 0,
            prev_gamepad: 0,
            prev_mouse: 0,
            bombs: HashMap::new(),
            snake,
            rng,
            enemies1: HashMap::new(),
            space,
            enemy_color: COLOR1,
            en_frame,
            difficulty,
            en_col_frame,
            score,
            high_score,
            multiplier,
            diff_mul_spike,
            respite,
            killer,
            death_countdown,
            score_next_life,
            palette_n,
            show_htp,
            show_game_over,
        }
    }

    pub fn input(&mut self, movement_enabled: bool) {
        let gamepad = unsafe { *wasm4::GAMEPAD1 };
        let just_pressed_gamepad = gamepad & (gamepad ^ self.prev_gamepad);
        self.snake.stop();
        if movement_enabled && gamepad & wasm4::BUTTON_LEFT != 0 {
            self.snake.left();
        }
        if movement_enabled && gamepad & wasm4::BUTTON_RIGHT != 0 {
            self.snake.right();
        }
        if movement_enabled && gamepad & wasm4::BUTTON_UP != 0 {
            self.snake.up();
        }
        if movement_enabled && gamepad & wasm4::BUTTON_DOWN != 0 {
            self.snake.down();
        }
        if movement_enabled && just_pressed_gamepad & wasm4::BUTTON_1 != 0 {
            // X
            self.snake.switch_color();
        }

        let mouse = unsafe { *MOUSE_BUTTONS };
        let mouse_x = unsafe { *MOUSE_X };
        let mouse_y = unsafe { *MOUSE_Y };

        let just_pressed_mouse = mouse & (mouse ^ self.prev_mouse);
        if movement_enabled && mouse & MOUSE_LEFT != 0 {
            let new_d_x =
                mouse_x as f64 - self.snake.get_position().x - self.snake.get_size() as f64 / 2.0
                    + 1.0;
            let new_d_y =
                mouse_y as f64 - self.snake.get_position().y - self.snake.get_size() as f64 / 2.0
                    + 1.0;
            if (new_d_x.abs() > 1.0 || new_d_y.abs() > 1.0)
            // To work on mobile (where the DPAD is on the screen) limit the
            // pointer interaction to just around the play area. Otherwise the
            // D-pad and button presses would be interpreted as directions
                && mouse_x >= -20
                && mouse_x <= SCREEN_SIZE as i16 + 20
                && mouse_y >= -20
                && mouse_y <= SCREEN_SIZE as i16 + 20
            {
                self.snake.set_direction(Coord {
                    x: new_d_x,
                    y: new_d_y,
                })
            }
        }
        if movement_enabled && just_pressed_mouse & MOUSE_RIGHT != 0 {
            self.snake.switch_color();
        }
        if just_pressed_mouse & MOUSE_MIDDLE != 0 || just_pressed_gamepad & BUTTON_2 != 0 {
            self.palette_n = (self.palette_n + 1) % PALETTES.len() as u8;
            palette::set_palette_n(self.palette_n as usize);
        }
        if just_pressed_gamepad & wasm4::BUTTON_1 != 0 || just_pressed_mouse & MOUSE_LEFT != 0 {
            // X
            if self.show_htp {
                self.show_htp = false;
            }
            if self.show_game_over {
                self.save_and_restart();
                self.show_game_over = false;
            }
        }

        self.prev_gamepad = gamepad;
        self.prev_mouse = mouse;
    }

    pub fn update(&mut self) {
        self.draw_space();
        self.input(!self.death_happened());

        if self.show_htp {
            htp_screen();
            return;
        }
        set_draw_color(0x12);
        text(self.score.to_string(), 1, 1);
        text(
            "H:".to_string() + self.high_score.to_string().as_str(),
            73,
            1,
        );
        text(
            "x".to_string() + self.multiplier.to_string().as_str(),
            1,
            SCREEN_SIZE as i32 - 8,
        );
        // text(
        //     "LVL:".to_string() + (self.difficulty + 1).to_string().as_str(),
        //     60,
        //     SCREEN_SIZE as i32 - 8,
        // );
        set_draw_color(0x20);
        let h_start: i32 = SCREEN_SIZE as i32 - 9;
        for l in 0..self.snake.get_life() {
            blit(
                &HEART,
                h_start - l * 8,
                SCREEN_SIZE as i32 - 9,
                8,
                8,
                BLIT_1BPP,
            );
        }
        self.frame_count += 1;
        // Set difficulty
        for (i, mul) in self.diff_mul_spike.iter().enumerate() {
            if self.multiplier < *mul {
                self.difficulty = max(i as u8, self.difficulty);
                break;
            }
        }
        if self.death_happened() {
            self.death_tick();
            return;
        }
        if self.snake.get_life() <= 0 {
            self.game_over_tick();
            return;
        }
        // Change generated enemy color
        if self.frame_count % self.en_col_frame[self.difficulty as usize] as u32 == 0 {
            if self.enemy_color == COLOR2 {
                self.enemy_color = COLOR1;
            } else {
                self.enemy_color = COLOR2;
            }
        }
        if self.frame_count % BOMB_FRAME_FREQ == 0 && self.bombs.len() < MAX_BOMBS {
            self.bombs.insert(
                self.frame_count,
                Box::new((
                    Bomb::new(&Coord {
                        x: self.rng.f64() * (SCREEN_SIZE - 20) as f64 + 10.0,
                        y: self.rng.f64() * (SCREEN_SIZE - 20) as f64 + 10.0,
                    }),
                    false,
                )),
            );
        }
        self.respite = max(self.respite - 1, 0);
        // Generate enemies
        if self.frame_count % self.en_frame[self.difficulty as usize] as u32 == 0
            && self.enemies1.len() < MAX_ENEMIES
            && self.respite == 0
        {
            let positions = [
                (0, 0),
                (0, SCREEN_SIZE / 2),
                (0, SCREEN_SIZE - 1),
                (SCREEN_SIZE / 2, 0),
                (SCREEN_SIZE / 2, SCREEN_SIZE - 1),
                (SCREEN_SIZE - 1, 0),
                (SCREEN_SIZE - 1, SCREEN_SIZE / 2),
                (SCREEN_SIZE - 1, SCREEN_SIZE - 1),
            ];
            let pos = positions[self.rng.usize(0..positions.len())];

            self.enemies1.insert(
                self.frame_count,
                Box::new(Enemy1::new(
                    self.frame_count,
                    pos.0 as f64,
                    pos.1 as f64,
                    self.enemy_color,
                )),
            );
        }

        self.snake.update_position();

        for (_, enemy) in self.enemies1.iter_mut() {
            enemy.follow(&self.snake);
            enemy.update_position();
        }

        self.snake.draw();

        let mut to_delete: Vec<u32> = vec![];
        let mut snake_died = false;
        let mut killer: Option<Enemy1> = None;
        'outer: for (_, enemy) in self.enemies1.iter_mut() {
            for boxed in self.bombs.values() {
                if boxed.1 && boxed.0.collided_with_enemy(enemy) {
                    // enemy.kill();
                    enemy.set_color(self.snake.get_color())
                }
            }
            // Snake eats enemy
            if enemy.collided_with(&self.snake) {
                if enemy.get_color() == self.snake.get_color() {
                    enemy.kill();
                    self.score = (self.score + self.multiplier).clamp(0, 999_999_999);
                    self.high_score = max(self.score, self.high_score);
                    self.multiplier += 1;
                } else {
                    // Snake dies
                    snake_died = true;
                    killer = Some(Enemy1::new(
                        0,
                        enemy.get_position().x,
                        enemy.get_position().y,
                        enemy.get_color(),
                    ));
                    break 'outer;
                }
            }

            if enemy.life() <= 0 {
                to_delete.push(enemy.id());
            }

            if enemy.life() > 0 {
                enemy.draw();
            }
        }
        if snake_died {
            self.snake.set_life(self.snake.get_life() - 1);
            self.killer = killer;
            self.enemies1.drain();
        }

        for td in to_delete {
            self.enemies1.remove(&td);
        }

        let mut to_delete: Vec<u32> = vec![];
        for (id, boxed) in self.bombs.iter_mut() {
            if boxed.0.collided_with(&self.snake) {
                if !boxed.1 {
                    // Add extra points
                    self.score = (self.score + 10 * self.multiplier).clamp(0, 999_999_999);
                    self.high_score = max(self.score, self.high_score);
                    self.multiplier += 10;
                }
                boxed.1 = true;
            }
            if boxed.1 {
                // exploded
                boxed.0.grow();
            }
            if boxed.0.life() <= 0 {
                to_delete.push(*id);
            } else {
                boxed.0.draw();
            }
        }
        for td in to_delete {
            self.bombs.remove(&td);
        }
        if self.score > self.score_next_life {
            self.snake.set_life(self.snake.get_life() + 1);
            self.score_next_life *= 2;
        }
    }

    fn draw_space(&self) {
        set_draw_color(0x44);
        for p in &self.space {
            pixel(p.0 as i32, p.1 as i32);
        }
    }

    fn death_happened(&self) -> bool {
        // After every death, show just enemy that killed it and blink and stuff
        // and maybe give some respite
        self.killer.is_some()
    }
    fn death_tick(&mut self) {
        self.snake.draw();
        if self.frame_count % 20 == 0 {
            self.death_countdown -= 1;
        }
        match &self.killer {
            Some(killer) => {
                if self.death_countdown % 2 != 0 {
                    killer.draw()
                }
            }
            None => (),
        }
        if self.death_countdown <= 0 {
            self.killer = None;
            self.death_countdown = 6;
            self.respite = 180;
        }
    }

    fn game_over_tick(&mut self) {
        self.show_game_over = true;
        set_draw_color(0x14);
        text(
            "GAME OVER",
            SCREEN_SIZE as i32 / 2 - 35,
            SCREEN_SIZE as i32 / 2 - 10,
        );
        text("press X to restart", 8, SCREEN_SIZE as i32 / 2 + 15);
    }

    fn save_and_restart(&mut self) {
        let game_data: u32 = self.score;
        unsafe {
            let game_data_bytes = game_data.to_le_bytes();
            diskw(game_data_bytes.as_ptr(), core::mem::size_of::<u32>() as u32);
        }
        self.initialize_game();
    }
}
