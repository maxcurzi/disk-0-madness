use std::cmp::max;
use std::collections::HashMap;

use crate::bomb::Bomb;
use crate::draws::{self};
use crate::enemy::Enemy;
use crate::entity::{Coord, Movable, Visible};
use crate::music::{self, GAME_OVER_SONG, GAME_SONG_START, INTRO_SONG, VOICE_NOTES};
use crate::palette::{self, COLOR1, COLOR2, HEART};
use crate::player::Player;
use crate::screen;
use crate::wasm4::{
    blit, diskr, diskw, text, trace, BLIT_1BPP, BUTTON_1, BUTTON_2, BUTTON_DOWN, BUTTON_LEFT,
    BUTTON_RIGHT, BUTTON_UP, GAMEPAD1, MOUSE_BUTTONS, MOUSE_LEFT, MOUSE_MIDDLE, MOUSE_RIGHT,
    MOUSE_X, MOUSE_Y, SCREEN_SIZE,
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
const RESPITE_DURATION: usize = 120;
const DEATH_COUNTDOWN_DURATION: usize = 90;
// Enemy color switch times
const ENEMY_FRAME: [usize; DIFFICULTY_LEVELS] = [120, 60, 30, 25, 15, 10, 8, 6, 4, 2];
// Enemy spawn times
const EN_COL_FRAME: [usize; DIFFICULTY_LEVELS] = [240, 180, 160, 120, 100, 80, 60, 60, 60, 60];
// Score to difficulty
const DIFF_MUL_PROGRESSION: [u32; DIFFICULTY_LEVELS - 1] =
    [12, 30, 80, 120, 240, 320, 500, 1000, 2000];

/// Entities include the player, enemies and bombs
struct Entities {
    player: Player,
    bombs: HashMap<usize, Box<(Bomb, bool)>>,
    enemies: HashMap<usize, Box<Enemy>>,
    killer: Option<Enemy>, // stores enemy that killed player
}

impl Entities {
    fn new() -> Self {
        let mut player = Player::new();
        player.set_life(INIT_LIVES as u32);
        Self {
            player,
            bombs: HashMap::with_capacity(MAX_BOMBS),
            enemies: HashMap::with_capacity(MAX_ENEMIES),
            killer: None,
        }
    }
    fn update(&mut self) -> (u32, u32) {
        self.update_position();
        let (enemies_killed, bombs_exploded) = self.process_collisions();
        (enemies_killed, bombs_exploded)
    }

    fn update_position(&mut self) {
        self.player.update_position();
        self.player.stop();
        for (_, enemy) in self.enemies.iter_mut() {
            enemy.follow(&self.player);
            enemy.update_position();
        }
        for (_, bomb) in self.bombs.iter_mut() {
            if bomb.1 {
                bomb.0.grow();
            }
        }
        self.prune();
    }

    fn prune(&mut self) {
        let mut enemies_to_delete: Vec<usize> = vec![];
        for (&id, enemy) in self.enemies.iter_mut() {
            if enemy.life() == 0 {
                enemies_to_delete.push(id);
            }
        }
        for td in enemies_to_delete {
            self.enemies.remove(&td);
        }

        let mut bombs_to_delete: Vec<usize> = vec![];
        for (&id, bomb) in self.bombs.iter_mut() {
            if bomb.0.life() == 0 {
                bombs_to_delete.push(id);
            }
        }
        for td in bombs_to_delete {
            self.bombs.remove(&td);
        }
    }

    fn draw(&self) {
        self.player.draw();

        for enemy in self.enemies.values() {
            enemy.draw();
        }

        for bomb in self.bombs.values() {
            bomb.0.draw();
        }
    }

    fn process_collisions(&mut self) -> (u32, u32) {
        // Returned variables
        let mut enemies_killed = 0;
        let mut bombs_exploded = 0;

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
                    self.killer = Some(Enemy::new(
                        enemy.id(),
                        enemy.get_position().x,
                        enemy.get_position().y,
                        enemy.get_color(),
                    ));
                    enemy.kill();
                    break 'outer;
                }
            }
        }

        for (_, boxed) in self.bombs.iter_mut() {
            if boxed.0.collided_with_player(&self.player) && !boxed.1 {
                bombs_exploded += 1;
                boxed.1 = true;
            }
        }
        self.prune();

        (enemies_killed, bombs_exploded)
    }
}

/// Counters to keep track of time in music and some events
struct Timers {
    // The main game counter is frame_count (the frame counter) used as "tick"
    // in many operations, some timers are helpful to control the duration of
    // the death animation, or the time without enemies when the player spawns.
    //
    // Song tick is mainly used to start the "game" and "game_over" songs on the
    // first beat. The other main game song is composed of multiple "songs"
    // which are switched depending on the level, but it should be perceived as
    // a single song without beat changes. The synchronization is  handled in
    // the song player
    frame_count: usize,
    death_countdown: usize,
    respite: usize, // frames without enemies
    song_tick: usize,
}
impl Timers {
    fn new() -> Self {
        Self {
            frame_count: 0,
            death_countdown: DEATH_COUNTDOWN_DURATION,
            respite: RESPITE_DURATION,
            song_tick: 0,
        }
    }

    fn tick(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
        // self.death_countdown = self.death_countdown.saturating_sub(1);
        self.respite = self.respite.saturating_sub(1);
        self.song_tick = self.song_tick.wrapping_add(1);
    }
}

/// Calibrations impact the gameplay difficultym, randomness, when the player
/// gets extra lives, etc...
struct Calibrations {
    difficulty: u32,
    score_next_life: u32,
    rng: Rng,
    enemy_color: u16,
}

impl Calibrations {
    fn new(tick_for_extra_rng: usize) -> Self {
        Self {
            difficulty: INIT_DIFFICULTY,
            score_next_life: NEXT_LIFE_SCORE,
            rng: Rng::with_seed(RNG_SEED + tick_for_extra_rng as u64),
            enemy_color: COLOR2,
        }
    }
}

/// Score simply depends on enemies absorbed and bombs exploded. Each enemy/bomb
/// gives an increasing amount of score, defined by the multiplier.
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

    /// Updates the player's score depening on how many enemies were killed and
    /// bombs exploded in the current frame. Increase the multiplier every time
    /// something happens. Bombs are worth a lot more.
    fn update(&mut self, enemies_killed: u32, bombs_exploded: u32) {
        for _ in 0..bombs_exploded {
            self.current = self.current.wrapping_add(self.multiplier.wrapping_mul(10));
            self.multiplier = self.multiplier.wrapping_add(10);
        }
        for _ in 0..enemies_killed {
            self.current = self.current.wrapping_add(self.multiplier);
            self.multiplier = self.multiplier.wrapping_add(1);
        }
        self.current = self.current.clamp(0, 999_999_999);
    }
}

/// Flags are here used mainly to control which screen needs to be shown and
/// little else. There's probably a simpler way of handling screens and
/// transitions but given the game has a very linear structure the approach
/// works fine.
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

/// The Environment is responsible for drawing the play area (space), adjust
/// palette colours, and play music and sound effects. The HUD (score,
/// lives, etc..) is not part of the enviroment.
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
            song_nr: INTRO_SONG,
        }
    }

    fn draw_space(&self) {
        palette::set_draw_color(0x44);
        for p in &self.space {
            draws::pixel(p.0 as i32, p.1 as i32);
        }
    }

    fn update(&mut self, _tick: usize, song_tick: usize) {
        music::play_music(song_tick / MUSIC_SPEED_CTRL as usize, self.song_nr);
        self.draw_space();
    }

    fn play_sound_effects(&self, bombs_exploded: bool, extra_life: bool, player_died: bool) {
        // We just have very few sound effects.
        if bombs_exploded {
            music::bomb_sound();
        }
        if extra_life {
            music::extra_life_sound();
        }
        if player_died {
            music::death_sound();
        }
    }

    fn set_palette(&mut self, palette_nr: u8) {
        self.palette_n = palette_nr % palette::PALETTES.len() as u8;
        palette::set_palette_n(self.palette_n as usize)
    }
}

/// Handles user actions (mainly keyboard and mouse actions)
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
    const MOUSE_AREA_PADDING: i16 = 20; // Extra space around play area to allow mouse events.
    fn new() -> Self {
        Self {
            prev_gamepad: unsafe { *GAMEPAD1 },
            prev_mouse: unsafe { *MOUSE_BUTTONS },
        }
    }

    /// Read from peripherals and return everything that's happening
    fn update(&mut self) -> Vec<ControlEvent> {
        // Return value
        let mut event = vec![];

        // Local vars
        let gamepad = unsafe { *GAMEPAD1 };
        let just_pressed_gamepad = gamepad & (gamepad ^ self.prev_gamepad);
        let mouse = unsafe { *MOUSE_BUTTONS };
        let just_pressed_mouse = mouse & (mouse ^ self.prev_mouse);

        // Check gamepad
        if gamepad & BUTTON_LEFT != 0 {
            event.push(ControlEvent::KbdLeft);
        }
        if gamepad & BUTTON_DOWN != 0 {
            event.push(ControlEvent::KbdDown);
        }
        if gamepad & BUTTON_UP != 0 {
            event.push(ControlEvent::KbdUp);
        }
        if gamepad & BUTTON_RIGHT != 0 {
            event.push(ControlEvent::KbdRight);
        }
        if just_pressed_gamepad & BUTTON_1 != 0 {
            event.push(ControlEvent::KbdBtn1);
        }
        if just_pressed_gamepad & BUTTON_2 != 0 {
            event.push(ControlEvent::KbdBtn2);
        }

        // Check mouse
        if mouse & MOUSE_RIGHT != 0
            && self.mouse_in_play_area_within_padding(Self::MOUSE_AREA_PADDING)
        {
            let mouse_x = unsafe { *MOUSE_X };
            let mouse_y = unsafe { *MOUSE_Y };
            event.push(ControlEvent::MouseRightHold((mouse_x, mouse_y)));
        }
        if just_pressed_mouse & MOUSE_LEFT != 0
            && self.mouse_in_play_area_within_padding(Self::MOUSE_AREA_PADDING)
        {
            event.push(ControlEvent::MouseLeftClick);
        }
        if just_pressed_mouse & MOUSE_MIDDLE != 0
            && self.mouse_in_play_area_within_padding(Self::MOUSE_AREA_PADDING)
        {
            event.push(ControlEvent::MouseMiddleClick);
        }

        self.prev_gamepad = gamepad;
        self.prev_mouse = mouse;

        event
    }

    /// This restricts the mouse action area within the game area
    /// (with some padding outside).
    ///
    /// This is useful for multiple reasons:
    /// 1. Clicking on dev tools should not be registered as a game event, if
    ///    the dev tools area fall outside the play area
    /// 2. When playing on mobile all phone area counts as mouse area and
    ///    this makes it impossible to use the DPAD (because when the DPAD or
    ///    buttons are tapped, they are also detected as mouse events)
    ///
    /// The padding is useful to allow the player to use the mouse/fingers
    /// slightly outside the play area and still register inputs. It's
    /// frustrating to lose control of the disk and die if your mouse pointer
    /// was ever so slighty outside!
    fn mouse_in_play_area_within_padding(&self, padding: i16) -> bool {
        let mouse_x = unsafe { *MOUSE_X };
        let mouse_y = unsafe { *MOUSE_Y };
        mouse_x >= -padding
            && mouse_x <= SCREEN_SIZE as i16 + padding
            && mouse_y >= -padding
            && mouse_y <= SCREEN_SIZE as i16 + padding
    }
}

/// This is where everything comes together.
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
        let calibrations = Calibrations::new(0);
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

    /// A game restarts when the player runs out of lives and decides to play
    /// again. Use a new random seed for the rng, to keep the universe fresh.
    pub fn restart(&mut self) {
        self.entities = Entities::new();
        self.calibrations = Calibrations::new(self.timers.frame_count);
        self.environment = Environment::new(&self.calibrations.rng);
        self.timers = Timers::new();
        self.scores = Scores::new();
        self.flags = Flags::new();
        self.flags.show_title = false;
        self.flags.show_htp = false;
        let mut player = Player::new();
        player.set_life(INIT_LIVES as u32);

        self.entities.player = player;
    }

    /// Read what actions the user has done and update the game accordingly.
    pub fn process_inputs(&mut self) {
        let control_events = self.controls.update();
        let mut continue_action = false;
        let movement_enabled = self.entities.killer.is_none();
        for event in control_events {
            match event {
                ControlEvent::KbdLeft => {
                    if movement_enabled {
                        self.entities.player.left();
                    }
                }
                ControlEvent::KbdDown => {
                    if movement_enabled {
                        self.entities.player.down();
                    }
                }
                ControlEvent::KbdUp => {
                    if movement_enabled {
                        self.entities.player.up();
                    }
                }
                ControlEvent::KbdRight => {
                    if movement_enabled {
                        self.entities.player.right();
                    }
                }
                ControlEvent::KbdBtn1 | ControlEvent::MouseLeftClick => {
                    if movement_enabled {
                        self.entities.player.switch_color();
                    }
                    continue_action = true;
                }
                ControlEvent::KbdBtn2 | ControlEvent::MouseMiddleClick => {
                    self.environment.set_palette(self.environment.palette_n + 1);
                }
                ControlEvent::MouseRightHold((mouse_x, mouse_y)) => {
                    if movement_enabled {
                        let new_d_x = mouse_x as f64
                            - self.entities.player.get_position().x
                            - self.entities.player.get_size() as f64 / 2.0;
                        let new_d_y = mouse_y as f64
                            - self.entities.player.get_position().y
                            - self.entities.player.get_size() as f64 / 2.0;
                        if new_d_x.abs() > 1.0 || new_d_y.abs() > 1.0 {
                            self.entities.player.set_direction(Coord {
                                x: new_d_x,
                                y: new_d_y,
                            })
                        }
                    }
                }
            }
        }

        // In a non-playing screen, waiting for input (pretty much the start
        // screen and the "Press X to start/continue" ones)
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

    /// Called at every frame. It's the beating heart of the game. It must call
    /// self.timers.tick() exactly once before returning.
    pub fn update(&mut self) {
        self.environment
            .update(self.timers.frame_count, self.timers.song_tick);
        self.process_inputs();

        // First we show the title screen
        if self.flags.show_title {
            screen::title(self.timers.frame_count);
            self.timers.tick();
            return;
        }
        // Then how to play
        if self.flags.show_htp {
            screen::how_to_play(self.timers.frame_count);
            self.timers.tick();
            return;
        }

        // When the game starts the HUD will be always visible
        self.draw_hud();
        // Game over screen
        if self.flags.show_game_over {
            screen::game_over(self.timers.frame_count);
            self.timers.tick();
            return;
        }

        // Stop-the-world death event
        if self.entities.killer.is_some() {
            self.death_tick();
            self.timers.tick();
            return;
        }

        // End-game. Player ran out of lives, save high score and flag for game-over
        if self.entities.player.get_life() == 0 {
            self.flags.new_high_score = self.scores.current > self.scores.high;
            self.scores.high = max(self.scores.current, self.scores.high);
            self.flags.show_game_over = true;
            self.environment.song_nr = GAME_OVER_SONG as u8;
            self.timers.song_tick = 0;

            // Save high score
            let game_data: u32 = self.scores.current;
            unsafe {
                let game_data_bytes = game_data.to_le_bytes();
                diskw(game_data_bytes.as_ptr(), core::mem::size_of::<u32>() as u32);
            }
            self.timers.tick();
            return;
        }

        self.update_difficulty();
        let (enemies_killed, bombs_exploded) = self.entities.update();

        self.update_score(enemies_killed, bombs_exploded);

        if self.entities.killer.is_some() {
            self.entities
                .player
                .set_life(self.entities.player.get_life().saturating_sub(1));
        }

        self.spawn_enemies();
        self.spawn_bombs();
        self.entities.draw();

        self.sounds_and_music_tick(bombs_exploded);

        // Print Statistics
        #[cfg(debug_assertions)]
        self.print_statistics();

        self.timers.tick();
    }

    fn print_statistics(&mut self) {
        if self.timers.frame_count % 60 == 0 {
            trace(
                "Enemies:".to_owned()
                    + self.entities.enemies.len().to_string().as_str()
                    + "/"
                    + self.entities.enemies.capacity().to_string().as_str(),
            );
        }
    }

    fn sounds_and_music_tick(&mut self, bombs_exploded: u32) {
        // Play relevant sounds
        self.environment.play_sound_effects(
            bombs_exploded > 0,
            self.scores.current > self.calibrations.score_next_life,
            self.entities.killer.is_some(),
        );
        // Update music appropriately with difficulty level
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
    }

    fn update_score(&mut self, enemies_killed: u32, bombs_exploded: u32) {
        self.scores.update(enemies_killed, bombs_exploded);
        if self.scores.current > self.calibrations.score_next_life {
            self.entities
                .player
                .set_life(self.entities.player.get_life().saturating_add(1));
            self.calibrations.score_next_life = self.calibrations.score_next_life.saturating_mul(2);
        }
    }

    fn update_difficulty(&mut self) {
        // Set difficulty. It simply depends on the current multiplier.
        for (i, mul) in DIFF_MUL_PROGRESSION.iter().enumerate() {
            if self.scores.multiplier < *mul {
                self.calibrations.difficulty = max(i as u32, self.calibrations.difficulty);
                break;
            }
        }
    }

    fn spawn_bombs(&mut self) {
        // Bombs are spawned with a similar logic to the enemies, but in random positions on screen.
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
    }
    fn spawn_enemies(&mut self) {
        // Enemy color depends on time, so we can have nice sections of enemies
        // with same colour, while keeping some element of randomness (their position).
        if self.timers.frame_count % EN_COL_FRAME[self.calibrations.difficulty as usize] as usize
            == 0
        {
            if self.calibrations.enemy_color == COLOR2 {
                self.calibrations.enemy_color = COLOR1;
            } else {
                self.calibrations.enemy_color = COLOR2;
            }
        }
        // We only spawn a maximum of 1 enemy per frame, at an interval decided
        // by ENEMY_FRAME. It works fine and even at 1 enemy per frame (60
        // enemies per second) the pressure is high. Consider adjusting to spawn
        // multiple enemies per frame which may be more visually pleasing.
        //
        // Enemies are randomly spawned at 8 fixed locations (corners and mid-edges)
        if self.timers.frame_count % ENEMY_FRAME[self.calibrations.difficulty as usize] as usize
            == 0
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
    }
    fn draw_hud(&self) {
        // Draws score, high-score, multiplier, player life count
        palette::set_draw_color(0x12);
        text(self.scores.current.to_string(), 1, 1);
        if self.flags.new_high_score
            && self.flags.show_game_over
            && (self.timers.frame_count / 2) % 10 < 5
        {
            palette::set_draw_color(0x00);
        }
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
        // #[cfg(debug_assertions)]
        // text(
        //     "LVL:".to_string() + (self.calibrations.difficulty + 1).to_string().as_str(),
        //     60,
        //     SCREEN_SIZE as i32 - 8,
        // );
        palette::set_draw_color(0x20);
        let h_start: i32 = SCREEN_SIZE as i32 - 9;
        for l in 0..self.entities.player.get_life() {
            blit(
                &HEART,
                h_start - l as i32 * 8,
                SCREEN_SIZE as i32 - 9,
                8,
                8,
                BLIT_1BPP,
            );
        }
    }

    fn death_tick(&mut self) {
        // Just shows player and blink killer
        self.entities.player.draw();

        match &self.entities.killer {
            Some(killer) => {
                if self.timers.death_countdown / 10 % 2 != 0 {
                    killer.draw()
                }
            }
            None => (),
        }
        self.timers.death_countdown = self.timers.death_countdown.saturating_sub(1);
        if self.timers.death_countdown == 0 {
            self.entities.enemies.clear();
            self.entities.killer = None;
            self.timers.death_countdown = DEATH_COUNTDOWN_DURATION;
            self.timers.respite = RESPITE_DURATION;
        }
    }
}
