use std::cmp::max;
use std::collections::HashMap;

use crate::bomb::Bomb;
use crate::draws::pixel;
use crate::enemy::Enemy1;
use crate::entity::{Coord, Movable, Visible};
use crate::music::{bomb_sound, death_sound, extra_life_sound, music_player, VOICE_NOTES};
use crate::palette::{self, COLOR1, COLOR2, HEART, PALETTES};
use crate::snake::Snake1;
use crate::start_screen::{game_over_screen, htp_screen, title_screen};
use crate::wasm4::{
    self, blit, diskr, diskw, text, trace, BLIT_1BPP, BUTTON_2, GAMEPAD1, MOUSE_BUTTONS,
    MOUSE_LEFT, MOUSE_MIDDLE, MOUSE_RIGHT, MOUSE_X, MOUSE_Y, SCREEN_SIZE,
};

use fastrand::Rng;

const MAX_ENEMIES: usize = 200;
const MAX_BOMBS: usize = 16;
const INIT_LIVES: usize = 3;
const INIT_DIFFICULTY: usize = 0;
const BOMB_FRAME_FREQ: usize = 400;
const MUSIC_SPEED_CTRL: usize = 5;
const DIFFICULTY_LEVELS: usize = 10;

pub struct Game {
    rng: Rng,
    frame_count: usize,
    snake: Snake1,
    prev_gamepad: u8,
    prev_mouse: u8,
    bombs: HashMap<usize, Box<(Bomb, bool)>>,
    enemies1: HashMap<usize, Box<Enemy1>>,
    space: Vec<(u8, u8)>,
    enemy_color: u16,
    en_frame: [usize; DIFFICULTY_LEVELS],
    difficulty: usize,
    en_col_frame: [usize; DIFFICULTY_LEVELS],
    score: u32,
    high_score: u32,
    multiplier: u32,
    diff_mul_progression: [u32; DIFFICULTY_LEVELS], //Vec<u32>,
    respite: i32,                                   // frames without enemies
    killer: Option<Enemy1>,                         // store enemy that killed player
    death_countdown: i32,
    score_next_life: u32,
    palette_n: u8,
    show_htp: bool,
    show_title: bool,
    show_game_over: bool,
    song_nr: u8,
}

impl Game {
    pub fn initialize_game(&mut self) {
        self.difficulty = INIT_DIFFICULTY;
        self.score = 0;
        self.multiplier = 1;
        let mut snake = Snake1::new();
        snake.set_life(INIT_LIVES as i32);
        self.snake = snake;
        self.enemies1.clear();
        self.bombs.clear();
        self.respite = 0;
        self.killer = None;
        self.death_countdown = 6;
        self.score_next_life = 100_000;
        self.palette_n = 0;
        self.show_htp = false;
        self.show_title = false;
        self.show_game_over = false;
        self.song_nr = 1;
    }
    pub fn new() -> Self {
        let rng = Rng::with_seed(555);
        let en_frame: [usize; 10] = [120, 60, 30, 25, 15, 10, 8, 6, 4, 2];
        let en_col_frame: [usize; 10] = [240, 180, 160, 120, 100, 80, 60, 60, 60, 60];
        // let diff_mul_spike: [u32; 10] = [4, 20, 50, 80, 100, 130, 160, 200, 250, 1500];
        let diff_mul_progression: [u32; 10] = [4, 20, 50, 100, 200, 400, 600, 900, 1200, 1500];
        let difficulty = INIT_DIFFICULTY;
        let score: u32 = 0;
        let respite = 0;
        let multiplier = 1;
        let killer: Option<Enemy1> = None;
        let death_countdown = 6;
        let score_next_life = 100_000;
        let palette_n = 0;
        let show_htp = true;
        let show_title = true;
        let show_game_over = false;
        let song_nr = 0;
        const SPACE_PIXELS: u8 = 200;
        let mut space = vec![];
        let bombs = HashMap::with_capacity(MAX_BOMBS);
        let enemies1 = HashMap::with_capacity(MAX_ENEMIES);
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
        snake.set_life(INIT_LIVES as i32);

        Self {
            frame_count: 0,
            prev_gamepad: 0,
            prev_mouse: 0,
            bombs,
            snake,
            rng,
            enemies1,
            space,
            enemy_color: COLOR1,
            en_frame,
            difficulty,
            en_col_frame,
            score,
            high_score,
            multiplier,
            diff_mul_progression,
            respite,
            killer,
            death_countdown,
            score_next_life,
            palette_n,
            show_htp,
            show_title,
            show_game_over,
            song_nr,
        }
    }

    pub fn input(&mut self, movement_enabled: bool) {
        let gamepad = unsafe { *GAMEPAD1 };
        let just_pressed_gamepad = gamepad & (gamepad ^ self.prev_gamepad);

        let mouse = unsafe { *MOUSE_BUTTONS };
        let mouse_x = unsafe { *MOUSE_X };
        let mouse_y = unsafe { *MOUSE_Y };
        let just_pressed_mouse = mouse & (mouse ^ self.prev_mouse);

        self.snake.stop();

        // Adjust snake direction/movement
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

        // Player color switch
        if movement_enabled
            && ((just_pressed_gamepad & wasm4::BUTTON_1 != 0)
                || (just_pressed_mouse & MOUSE_RIGHT != 0))
        {
            // X or Click
            self.snake.switch_color();
        }

        // Palette change
        if just_pressed_mouse & MOUSE_MIDDLE != 0 || just_pressed_gamepad & BUTTON_2 != 0 {
            self.palette_n = (self.palette_n + 1) % PALETTES.len() as u8;
            palette::set_palette_n(self.palette_n as usize);
        }

        // In a non-playing screen, waiting for input
        if (self.show_title || self.show_htp || self.show_game_over)
            && (just_pressed_gamepad & wasm4::BUTTON_1 != 0 || just_pressed_mouse & MOUSE_LEFT != 0)
        {
            // X
            if !self.show_title && self.show_htp {
                self.show_htp = false;
                self.initialize_game();
            }
            if self.show_title {
                self.show_title = false;
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
        self.frame_count += 1;
        music_player(self.frame_count / MUSIC_SPEED_CTRL as usize, self.song_nr);

        self.draw_space();
        self.input(!self.death_happened());
        if self.show_title {
            title_screen(self.frame_count as usize);
            return;
        }
        if self.show_htp {
            htp_screen(self.frame_count as usize);
            return;
        }

        palette::set_draw_color(0x12);
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
        #[cfg(debug_assertions)]
        text(
            "LVL:".to_string() + (self.difficulty + 1).to_string().as_str(),
            60,
            SCREEN_SIZE as i32 - 8,
        );
        palette::set_draw_color(0x20);
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
        if self.death_happened() {
            self.death_tick();
            return;
        }

        if self.snake.get_life() <= 0 {
            self.show_game_over = true;
            self.song_nr = 0;
            game_over_screen(self.frame_count as usize);
            return;
        }

        // Set difficulty
        for (i, mul) in self.diff_mul_progression.iter().enumerate() {
            if self.multiplier < *mul {
                self.difficulty = max(i, self.difficulty);
                break;
            }
        }

        // Change generated enemy color
        if self.frame_count % self.en_col_frame[self.difficulty as usize] as usize == 0 {
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
        if self.frame_count % self.en_frame[self.difficulty as usize] as usize == 0
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

        let mut to_delete: Vec<usize> = vec![];
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
            #[cfg(debug_assertions)]
            trace("Snake died");
            self.snake.set_life(self.snake.get_life() - 1);
            self.killer = killer;
            self.enemies1.clear();
            death_sound();
        }
        let mut deleted = 0;
        for td in to_delete {
            deleted += 1;
            self.enemies1.remove(&td);
        }
        if deleted > 0 {
            #[cfg(debug_assertions)]
            trace("Deleted:".to_owned() + deleted.to_string().as_str());
        }
        let mut to_delete: Vec<usize> = vec![];
        for (id, boxed) in self.bombs.iter_mut() {
            if boxed.0.collided_with(&self.snake) {
                if !boxed.1 {
                    // Add extra points
                    // trace("bomb");
                    self.score = (self.score + 10 * self.multiplier).clamp(0, 999_999_999);
                    self.high_score = max(self.score, self.high_score);
                    self.multiplier += 10;
                    bomb_sound();
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
            // trace("extra life");
            self.snake.set_life(self.snake.get_life() + 1);
            self.score_next_life *= 2;
            extra_life_sound();
        }

        // Update music appropriately
        if ((self.frame_count + 1) / MUSIC_SPEED_CTRL) % VOICE_NOTES == 0 {
            match self.difficulty {
                0..=1 => self.song_nr = 1,
                2 => self.song_nr = 2,
                3 => self.song_nr = 3,
                4 => self.song_nr = 4,
                5 => self.song_nr = 5,
                _ => self.song_nr = 6,
            }
        }

        // Print Statistics
        #[cfg(debug_assertions)]
        if self.frame_count % 60 == 0 {
            trace(
                "Enemies:".to_owned()
                    + self.enemies1.len().to_string().as_str()
                    + "/"
                    + self.enemies1.capacity().to_string().as_str(),
            );
        }
    }

    fn draw_space(&self) {
        palette::set_draw_color(0x44);
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

    fn save_and_restart(&mut self) {
        let game_data: u32 = self.score;
        unsafe {
            let game_data_bytes = game_data.to_le_bytes();
            diskw(game_data_bytes.as_ptr(), core::mem::size_of::<u32>() as u32);
        }
        self.initialize_game();
    }
}
