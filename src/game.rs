use std::cmp::max;
use std::collections::HashMap;

use crate::bomb::Bomb;
use crate::draws::{self};
use crate::enemy::{self, Enemy};
use crate::entity::{Coord, Movable, Visible};
use crate::music::{
    bomb_sound, death_sound, extra_life_sound, music_player, GAME_OVER_SONG, GAME_SONG_START,
    INTRO_SONG, VOICE_NOTES,
};
use crate::palette::{self, COLOR1, COLOR2, HEART, PALETTES};
use crate::player::Player;
use crate::start_screen::{game_over_screen, htp_screen, title_screen};
use crate::wasm4::{
    self, blit, diskr, diskw, text, trace, BLIT_1BPP, BUTTON_2, GAMEPAD1, MOUSE_BUTTONS,
    MOUSE_LEFT, MOUSE_MIDDLE, MOUSE_RIGHT, MOUSE_X, MOUSE_Y, SCREEN_SIZE,
};

use fastrand::Rng;
const RNG_SEED: u64 = 555;
const MAX_ENEMIES: usize = 200;
const MAX_BOMBS: usize = 16;
const INIT_LIVES: usize = 3;
const INIT_DIFFICULTY: u32 = 0;
const BOMB_FRAME_FREQ: usize = 400;
const MUSIC_SPEED_CTRL: usize = 5;
const DIFFICULTY_LEVELS: usize = 10;
const NEXT_LIFE_SCORE: u32 = 100_000;
const RESPITE: usize = 150;
const EN_COL_FRAME: [usize; DIFFICULTY_LEVELS] = [120, 60, 30, 25, 15, 10, 8, 6, 4, 2]; // Enemy color switch times
const EN_FRAME: [usize; DIFFICULTY_LEVELS] = [240, 180, 160, 120, 100, 80, 60, 60, 60, 60]; // Enemy spawn times
const DIFF_MUL_PROGRESSION: [u32; DIFFICULTY_LEVELS - 1] =
    [12, 30, 60, 100, 220, 350, 500, 1000, 2000]; // Score to difficulty

struct Entities {
    player: Player,
    bombs: HashMap<usize, Box<(Bomb, bool)>>,
    enemies: HashMap<usize, Box<Enemy>>,
    killer: Option<Enemy>, // store enemy that killed player
}

impl Entities {
    fn new() -> Self {
        let mut player = Player::new();
        player.set_life(INIT_LIVES as i32);
        Self {
            player,
            bombs: HashMap::with_capacity(MAX_BOMBS),
            enemies: HashMap::with_capacity(MAX_ENEMIES),
            killer: None,
        }
    }
    fn update(&mut self) -> (u32, u32, Option<Enemy>) {
        self.update_position();
        let (enemies_killed, bombs_exploded, killer) = self.process_collisions();
        (enemies_killed, bombs_exploded, killer)
    }

    fn update_position(&mut self) {
        self.player.stop();
        self.player.update_position();

        for (_, enemy) in self.enemies.iter_mut() {
            enemy.follow(&self.player);
            enemy.update_position();
        }
        for (id, bomb) in self.bombs.iter_mut() {
            if bomb.1 {
                bomb.0.grow();
            }
        }
        self.prune();
    }

    fn prune(&mut self) {
        let mut enemies_to_delete: Vec<usize> = vec![];
        for (_, enemy) in self.enemies.iter_mut() {
            if enemy.life() <= 0 {
                enemies_to_delete.push(enemy.id());
            }
        }
        for td in enemies_to_delete {
            self.enemies.remove(&td);
        }

        let mut bombs_to_delete: Vec<usize> = vec![];
        for (_, bomb) in self.bombs.iter_mut() {
            if bomb.0.life() <= 0 {
                bombs_to_delete.push(bomb.0.id());
            }
            for td in bombs_to_delete {
                self.bombs.remove(&td);
            }
        }
    }

    fn draw(&self) {
        for enemy in self.enemies.values() {
            enemy.draw();
        }

        for (id, bomb) in self.bombs.iter_mut() {
            bomb.0.draw();
        }
    }

    fn process_collisions(&mut self) -> (u32, u32, Option<Enemy>) {
        // enemies killed, bombs exploded, killer){
        // Returned variables
        let mut enemies_killed = 0;
        let mut bombs_exploded = 0;
        let mut killer: Option<Enemy> = None;

        let mut enemies_to_delete: Vec<usize> = vec![];
        'outer: for (_, enemy) in self.enemies.iter_mut() {
            for boxed in self.bombs.values() {
                if boxed.1 && boxed.0.collided_with_enemy(enemy) {
                    enemy.set_color(self.player.get_color())
                }
            }
            if enemy.collided_with(&self.player) {
                if enemy.get_color() == self.player.get_color() {
                    // Player eats enemy
                    enemy.kill();
                    enemies_killed += 1;
                } else {
                    // Player dies
                    killer = Some(Enemy::new(
                        0,
                        enemy.get_position().x,
                        enemy.get_position().y,
                        enemy.get_color(),
                    ));
                    break 'outer;
                }
            }
        }

        for (id, boxed) in self.bombs.iter_mut() {
            if boxed.0.collided_with_player(&self.player) {
                boxed.1 = true;
                bombs_exploded += 1;
            }
        }
        self.prune();

        (enemies_killed, bombs_exploded, killer)
    }
}

struct Timers {
    frame_count: usize,
    death_countdown: usize,
    respite: usize, // frames without enemies
}
impl Timers {
    fn new() -> Self {
        Self {
            frame_count: 0,
            death_countdown: 6,
            respite: RESPITE,
        }
    }

    fn tick(&mut self) {
        self.frame_count += 1;
        // if self.timers.frame_count % 20 == 0 {
        //     self.death_countdown -= 1;
        // }
        // self.timers.respite
    }
}

struct Calibrations {
    difficulty: u32,
    score_next_life: u32,
    rng: Rng,
    enemy_color: u16,
}

impl Calibrations {
    fn new() -> Self {
        Self {
            difficulty: INIT_DIFFICULTY,
            score_next_life: NEXT_LIFE_SCORE,
            rng: Rng::with_seed(RNG_SEED),
            enemy_color: COLOR2,
        }
    }
}

struct Scores {
    current: u32,
    multiplier: u32,
    high: u32,
}
impl Scores {
    fn new() -> Self {
        let high_score = unsafe {
            let mut buffer = [0u8; core::mem::size_of::<u32>()];

            diskr(buffer.as_mut_ptr(), buffer.len() as u32);

            u32::from_le_bytes(buffer)
        };

        Self {
            current: 0,
            multiplier: 1,
            high: high_score,
        }
    }

    fn update(&mut self, enemies_killed: u32, bombs_exploded: u32) {
        for _ in 0..bombs_exploded {
            self.current += 10 * self.multiplier;
            self.multiplier += 10;
        }
        for _ in 0..enemies_killed {
            self.current += self.multiplier;
            self.multiplier += 1;
        }
        self.current.clamp(0, 999_999_999);
    }
}

struct Flags {
    show_htp: bool,
    show_title: bool,
    show_game_over: bool,
    new_high_score: bool,
}
impl Flags {
    fn new() -> Self {
        Self {
            show_htp: true,
            show_title: true,
            show_game_over: false,
            new_high_score: false,
        }
    }
}
struct Environment {
    space: Vec<(u8, u8)>,
    palette_n: u8,
    song_nr: u8,
}
impl Environment {
    fn new(rng: &Rng) -> Self {
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
            song_nr: GAME_SONG_START,
        }
    }

    fn draw_space(&self) {
        palette::set_draw_color(0x44);
        for p in &self.space {
            draws::pixel(p.0 as i32, p.1 as i32);
        }
    }

    fn update(&mut self, tick: &usize) {
        music_player(tick / MUSIC_SPEED_CTRL as usize, self.song_nr);
        self.draw_space();
    }
}

struct Controls {
    prev_gamepad: u8,
    prev_mouse: u8,
}
enum ControlEvent {
    KbdLeft,
    KbdDown,
    KbdUp,
    KbdRight,
    KbdBtn1,
    KbdBtn2,
    MouseRightHold((i16, i16)),
    MouseLeftClick,
    MouseMiddleClick,
}

impl Controls {
    fn new() -> Self {
        Self {
            prev_gamepad: unsafe { *GAMEPAD1 },
            prev_mouse: unsafe { *MOUSE_BUTTONS },
        }
    }

    fn update(&mut self) -> Vec<ControlEvent> {
        // Read from peripherals and return everything that's happening
        // Return value
        let event = vec![];

        // Local vars
        let gamepad = unsafe { *GAMEPAD1 };
        let just_pressed_gamepad = gamepad & (gamepad ^ self.prev_gamepad);
        let mouse = unsafe { *MOUSE_BUTTONS };
        let just_pressed_mouse = mouse & (mouse ^ self.prev_mouse);

        // Check gamepad
        if gamepad & wasm4::BUTTON_LEFT != 0 {
            event.push(ControlEvent::KbdLeft);
        }
        if gamepad & wasm4::BUTTON_DOWN != 0 {
            event.push(ControlEvent::KbdDown);
        }
        if gamepad & wasm4::BUTTON_UP != 0 {
            event.push(ControlEvent::KbdUp);
        }
        if gamepad & wasm4::BUTTON_RIGHT != 0 {
            event.push(ControlEvent::KbdRight);
        }
        if just_pressed_gamepad & wasm4::BUTTON_1 != 0 {
            event.push(ControlEvent::KbdBtn1);
        }
        if just_pressed_gamepad & wasm4::BUTTON_2 != 0 {
            event.push(ControlEvent::KbdBtn2);
        }

        // Check mouse
        if mouse & wasm4::MOUSE_RIGHT != 0 {
            let mouse_x = unsafe { *MOUSE_X };
            let mouse_y = unsafe { *MOUSE_Y };
            event.push(ControlEvent::MouseRightHold((mouse_x, mouse_y)));
        }
        if just_pressed_mouse & wasm4::MOUSE_LEFT != 0 {
            event.push(ControlEvent::MouseLeftClick);
        }
        if just_pressed_mouse & wasm4::MOUSE_MIDDLE != 0 {
            event.push(ControlEvent::MouseMiddleClick);
        }

        self.prev_gamepad = gamepad;
        self.prev_mouse = mouse;

        event
    }
}

pub struct Game {
    entities: Entities,
    timers: Timers,
    calibrations: Calibrations,
    scores: Scores,
    flags: Flags,
    environment: Environment,
    controls: Controls,
}

impl Game {
    pub fn new() -> Self {
        let entities = Entities::new();
        let timers = Timers::new();
        let calibrations = Calibrations::new();
        let scores = Scores::new();
        let flags = Flags::new();
        let environment = Environment::new(&calibrations.rng);
        let controls = Controls::new();

        Self {
            entities,
            timers,
            calibrations,
            scores,
            flags,
            environment,
            controls,
        }
    }

    pub fn restart(&mut self) {
        self.entities = Entities::new();
        self.timers = Timers::new();
        self.calibrations = Calibrations::new();
        self.scores = Scores::new();
        self.flags = Flags::new();

        let mut player = Player::new();
        player.set_life(INIT_LIVES as i32);

        self.entities.player = player;
    }

    pub fn process_inputs(&mut self, movement_enabled: bool) {
        let controlEvents = self.controls.update();
        let continue_action = false;
        for event in controlEvents {
            match event {
                ControlEvent::KbdLeft => {
                    if movement_enabled {
                        self.entities.player.left()
                    }
                }
                ControlEvent::KbdDown => {
                    if movement_enabled {
                        self.entities.player.down()
                    }
                }
                ControlEvent::KbdUp => {
                    if movement_enabled {
                        self.entities.player.up()
                    }
                }
                ControlEvent::KbdRight => {
                    if movement_enabled {
                        self.entities.player.right()
                    }
                }
                ControlEvent::KbdBtn1 | ControlEvent::MouseLeftClick => {
                    if movement_enabled {
                        self.entities.player.switch_color();
                    }
                    continue_action = true;
                }
                ControlEvent::KbdBtn2 | ControlEvent::MouseMiddleClick => {
                    self.environment.palette_n =
                        (self.environment.palette_n + 1) % PALETTES.len() as u8;
                    palette::set_palette_n(self.environment.palette_n as usize);
                }
                ControlEvent::MouseRightHold((mouse_x, mouse_y)) => {
                    if movement_enabled {
                        let new_d_x = mouse_x as f64
                            - self.entities.player.get_position().x
                            - self.entities.player.get_size() as f64 / 2.0;
                        let new_d_y = mouse_y as f64
                            - self.entities.player.get_position().y
                            - self.entities.player.get_size() as f64 / 2.0;
                        if (new_d_x.abs() > 1.0 || new_d_y.abs() > 1.0)
                        // To work on mobile (where the DPAD is on the screen) limit the
                        // pointer interaction to just around the play area. Otherwise the
                        // D-pad and button presses would be interpreted as directions
                            && mouse_x >= -20
                            && mouse_x <= SCREEN_SIZE as i16 + 20
                            && mouse_y >= -20
                            && mouse_y <= SCREEN_SIZE as i16 + 20
                        {
                            self.entities.player.set_direction(Coord {
                                x: new_d_x,
                                y: new_d_y,
                            })
                        }
                    }
                }
            }
        }

        // In a non-playing screen, waiting for input
        if (self.flags.show_title || self.flags.show_htp || self.flags.show_game_over)
            && continue_action
        {
            if !self.flags.show_title && self.flags.show_htp {
                self.flags.show_htp = false;
                self.restart();
            }
            if self.flags.show_title {
                self.flags.show_title = false;
            }
            if self.flags.show_game_over {
                self.restart();
                self.flags.show_game_over = false;
            }
        }
    }

    pub fn update(&mut self) {
        self.environment.update(&self.timers.frame_count);
        self.process_inputs(!self.death_happened());

        if self.flags.show_title {
            title_screen(self.timers.frame_count as usize);
            return;
        }
        if self.flags.show_htp {
            htp_screen(self.timers.frame_count as usize);
            return;
        }

        palette::set_draw_color(0x12);
        if self.flags.new_high_score
            && self.flags.show_game_over
            && (self.timers.frame_count / 5) % 10 < 4
        {
            palette::set_draw_color(0x00);
        }
        text(self.scores.current.to_string(), 1, 1);
        text(
            "H:".to_string() + self.scores.high.to_string().as_str(),
            73,
            1,
        );
        palette::set_draw_color(0x12);
        text(
            "x".to_string() + self.scores.multiplier.to_string().as_str(),
            1,
            SCREEN_SIZE as i32 - 8,
        );
        #[cfg(debug_assertions)]
        text(
            "LVL:".to_string() + (self.calibrations.difficulty + 1).to_string().as_str(),
            60,
            SCREEN_SIZE as i32 - 8,
        );
        palette::set_draw_color(0x20);
        let h_start: i32 = SCREEN_SIZE as i32 - 9;
        for l in 0..self.entities.player.get_life() {
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

        if self.entities.player.get_life() <= 0 {
            self.scores.high = max(self.scores.current, self.scores.high);
            self.flags.show_game_over = true;
            self.environment.song_nr = GAME_OVER_SONG as u8;

            // Save high score
            let game_data: u32 = self.scores.current;
            unsafe {
                let game_data_bytes = game_data.to_le_bytes();
                diskw(game_data_bytes.as_ptr(), core::mem::size_of::<u32>() as u32);
            }

            game_over_screen(self.timers.frame_count as usize);
            return;
        }

        // Set difficulty
        for (i, mul) in DIFF_MUL_PROGRESSION.iter().enumerate() {
            if self.scores.multiplier < *mul {
                self.calibrations.difficulty = max(i as u32, self.calibrations.difficulty);
                break;
            }
        }

        // Change generated enemy color
        if self.timers.frame_count % EN_COL_FRAME[self.calibrations.difficulty as usize] as usize
            == 0
        {
            if self.calibrations.enemy_color == COLOR2 {
                self.calibrations.enemy_color = COLOR1;
            } else {
                self.calibrations.enemy_color = COLOR2;
            }
        }
        if self.timers.frame_count % BOMB_FRAME_FREQ == 0 && self.entities.bombs.len() < MAX_BOMBS {
            self.entities.bombs.insert(
                self.timers.frame_count,
                Box::new((
                    Bomb::new(&Coord {
                        x: self.calibrations.rng.f64() * (SCREEN_SIZE - 20) as f64 + 10.0,
                        y: self.calibrations.rng.f64() * (SCREEN_SIZE - 20) as f64 + 10.0,
                    }),
                    false,
                )),
            );
        }
        self.timers.respite = max(self.timers.respite - 1, 0);
        // Generate enemies
        if self.timers.frame_count % EN_FRAME[self.calibrations.difficulty as usize] as usize == 0
            && self.entities.enemies.len() < MAX_ENEMIES
            && self.timers.respite == 0
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
            let pos = positions[self.calibrations.rng.usize(0..positions.len())];

            self.entities.enemies.insert(
                self.timers.frame_count,
                Box::new(Enemy::new(
                    self.timers.frame_count,
                    pos.0 as f64,
                    pos.1 as f64,
                    self.calibrations.enemy_color,
                )),
            );
        }
        self.entities.update_position();

        let (enemies_killed, bombs_exploded, killer) = self.entities.update();

        if bombs_exploded > 0 {
            bomb_sound();
        }

        self.scores.update(enemies_killed, bombs_exploded);

        if killer.is_some() {
            self.entities
                .player
                .set_life(self.entities.player.get_life() - 1);
        }

        self.entities.player.draw();

        if self.scores.current > self.calibrations.score_next_life {
            self.entities
                .player
                .set_life(self.entities.player.get_life() + 1);
            self.calibrations.score_next_life *= 2;
            extra_life_sound();
        }

        // Update music appropriately
        if ((self.timers.frame_count + 1) / MUSIC_SPEED_CTRL) % VOICE_NOTES == 0 {
            match self.calibrations.difficulty {
                0..=1 => self.environment.song_nr = GAME_SONG_START,
                2 => self.environment.song_nr = GAME_SONG_START + 1,
                3 => self.environment.song_nr = GAME_SONG_START + 2,
                4 => self.environment.song_nr = GAME_SONG_START + 3,
                5 => self.environment.song_nr = GAME_SONG_START + 4,
                6 => self.environment.song_nr = GAME_SONG_START + 5,
                _ => self.environment.song_nr = GAME_SONG_START + 6,
            }
        }

        // Print Statistics
        #[cfg(debug_assertions)]
        if self.timers.frame_count % 60 == 0 {
            trace(
                "Enemies:".to_owned()
                    + self.entities.enemies.len().to_string().as_str()
                    + "/"
                    + self.entities.enemies.capacity().to_string().as_str(),
            );
        }
        self.timers.frame_count += 1;
    }

    fn death_happened(&self) -> bool {
        // After every death, show just enemy that killed it and blink and stuff
        // and maybe give some respite
        self.entities.killer.is_some()
    }

    fn death_tick(&mut self) {
        self.entities.player.draw();

        match &self.entities.killer {
            Some(killer) => {
                if self.death_countdown % 2 != 0 {
                    killer.draw()
                }
            }
            None => (),
        }
        if self.timers.death_countdown <= 0 {
            self.entities.killer = None;
            self.timers.death_countdown = 6;
            self.timers.respite = 180;
        }
    }

    fn save_and_restart(&mut self) {
        self.restart();
    }
}
