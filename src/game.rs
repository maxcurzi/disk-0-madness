use crate::{
    calibrations::{
        Calibrations, BOMB_FRAME_FREQ, DEATH_COUNTDOWN_DURATION, DIFF_MUL_PROGRESSION, ENEMY_FRAME,
        EN_COL_FRAME, INIT_LIVES, MAX_BOMBS, MAX_ENEMIES, MUSIC_SPEED_CTRL, RESPITE_DURATION,
    },
    common_types::Coord,
    controls::{ControlEvent, Controls},
    draws,
    entities::{
        bomb::Bomb,
        enemy::Enemy,
        manager::EntityManager as Entities,
        player::{Player, PlayerN},
        traits::Visible,
    },
    palette::{self, DRAW_COLOR_A, DRAW_COLOR_B, HEART},
    screen::{self, ScreenName},
    sound::{
        effects,
        music::{self, GAME_OVER_SONG, GAME_SONG_START, INTRO_SONG, VOICE_NOTES},
    },
    wasm4::{self, BLIT_1BPP, SCREEN_SIZE},
};
use fastrand::Rng;
use std::{borrow::BorrowMut, cmp::max};

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
        self.respite = self.respite.saturating_sub(1);
        self.song_tick = self.song_tick.wrapping_add(1);
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
        Self {
            current: 0,
            multiplier: 1,
            high: Scores::get_high_score(),
        }
    }
    fn get_high_score() -> u32 {
        unsafe {
            let mut buffer = [0u8; core::mem::size_of::<u32>()];
            wasm4::diskr(buffer.as_mut_ptr(), buffer.len() as u32);
            u32::from_le_bytes(buffer)
        }
    }
    fn save_high_score(&self, high_score: u32) {
        unsafe {
            let high_score_bytes = high_score.to_le_bytes();
            wasm4::diskw(
                high_score_bytes.as_ptr(),
                core::mem::size_of::<u32>() as u32,
            );
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
    current_screen: ScreenName,
    new_high_score: bool,
}
impl Flags {
    fn new() -> Self {
        Self {
            current_screen: ScreenName::Title,
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
        music::play(song_tick / MUSIC_SPEED_CTRL, self.song_nr);
        self.draw_space();
    }

    fn play_sound_effects(&self, bombs_exploded: bool, extra_life: bool, player_died: bool) {
        // We just have very few sound effects.
        if bombs_exploded {
            effects::bomb_explode();
        }
        if extra_life {
            effects::extra_life();
        }
        if player_died {
            effects::death();
        }
    }

    fn set_palette(&mut self, palette_nr: u8) {
        self.palette_n = palette_nr % palette::PALETTES.len() as u8;
        palette::set_palette_n(self.palette_n as usize)
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
        self.flags.current_screen = ScreenName::MainGame;
        self.entities.players[PlayerN::P1 as usize]
            .as_mut()
            .expect("Player should exist")
            .entity
            .life = INIT_LIVES;
    }

    /// Read what actions the user has done and update the game accordingly.
    pub fn process_inputs(&mut self) {
        let control_events = self.controls.update();
        let mut continue_action = false;
        let movement_enabled = self.entities.killer.is_none();

        for event in control_events {
            match event {
                ControlEvent::Left(player_n) => {
                    if movement_enabled && self.entities.players[player_n as usize].is_some() {
                        self.entities.players[player_n as usize]
                            .as_mut()
                            .expect("Player should exist")
                            .left();
                    }
                }
                ControlEvent::Down(player_n) => {
                    if movement_enabled && self.entities.players[player_n as usize].is_some() {
                        self.entities.players[player_n as usize]
                            .as_mut()
                            .expect("Player should exist")
                            .down();
                    }
                }
                ControlEvent::Up(player_n) => {
                    if movement_enabled && self.entities.players[player_n as usize].is_some() {
                        self.entities.players[player_n as usize]
                            .as_mut()
                            .expect("Player should exist")
                            .up();
                    }
                }
                ControlEvent::Right(player_n) => {
                    if movement_enabled && self.entities.players[player_n as usize].is_some() {
                        self.entities.players[player_n as usize]
                            .as_mut()
                            .expect("Player should exist")
                            .right();
                    }
                }
                ControlEvent::Btn1(player_n) => {
                    if movement_enabled && self.entities.players[player_n as usize].is_some() {
                        self.entities.players[player_n as usize]
                            .as_mut()
                            .expect("Player should exist")
                            .toggle_color();
                    }
                    // New player joins!
                    if self.entities.players[player_n as usize].is_none() {
                        wasm4::trace(
                            "Player ".to_owned()
                                + (player_n as u8 + 1).to_string().as_str()
                                + " joined!",
                        );
                        self.entities.players[player_n as usize] = Some(Player::new(player_n));
                        let lives = self.entities.players[PlayerN::P1 as usize]
                            .as_mut()
                            .expect("Player 1 should always exist")
                            .entity
                            .life
                            .borrow_mut();
                        *lives += INIT_LIVES;
                        effects::new_player();
                    }
                    continue_action = true;
                }

                ControlEvent::Btn2(player_n) => {
                    wasm4::trace(
                        "Player ".to_owned()
                            + (player_n as u8 + 1).to_string().as_str()
                            + " changed palette!",
                    );
                    self.environment.set_palette(self.environment.palette_n + 1);
                }

                ControlEvent::MouseRightClick => {
                    if movement_enabled {
                        if let Some(player) = self.entities.players[PlayerN::P1 as usize].as_mut() {
                            player.toggle_color();
                        }
                    }
                }
                ControlEvent::MouseLeftClick => {
                    continue_action = true;
                }
                ControlEvent::MouseMiddleClick => {
                    self.environment.set_palette(self.environment.palette_n + 1);
                }
                ControlEvent::MouseLeftHold((mouse_x, mouse_y)) => {
                    if movement_enabled {
                        if let Some(player) = self.entities.players[PlayerN::P1 as usize].as_mut() {
                            let new_d_x = mouse_x as f64
                                - player.entity.position.x
                                - player.entity.size / 2.0;
                            let new_d_y = mouse_y as f64
                                - player.entity.position.y
                                - player.entity.size / 2.0;
                            if new_d_x.abs() > 1.0 || new_d_y.abs() > 1.0 {
                                player.entity.direction = Coord {
                                    x: new_d_x,
                                    y: new_d_y,
                                };
                            }
                        }
                    }
                }
            }
        }

        // In a non-playing screen, waiting for input (pretty much the start
        // screen and the "Press X to start/continue" ones)
        if continue_action {
            match self.flags.current_screen {
                ScreenName::Title => self.flags.current_screen = ScreenName::HowToPlay,
                ScreenName::HowToPlay | ScreenName::GameOver => self.restart(),
                ScreenName::MainGame => (),
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
        if self.flags.current_screen == ScreenName::Title {
            screen::title(self.timers.frame_count);
            self.timers.tick();
            return;
        }
        // Then how to play
        if self.flags.current_screen == ScreenName::HowToPlay {
            screen::how_to_play(self.timers.frame_count);
            self.timers.tick();
            return;
        }

        // When the game starts the HUD will be always visible
        self.draw_hud();
        // Game over screen
        if self.flags.current_screen == ScreenName::GameOver {
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
        if self.entities.players[PlayerN::P1 as usize]
            .as_mut()
            .expect("Player should exist")
            .entity
            .life
            == 0
        {
            self.flags.new_high_score = self.scores.current > self.scores.high;
            self.scores.high = max(self.scores.current, self.scores.high);
            self.flags.current_screen = ScreenName::GameOver;
            self.environment.song_nr = GAME_OVER_SONG;
            self.timers.song_tick = 0;

            // Save high score
            let high_score: u32 = self.scores.high;
            self.scores.save_high_score(high_score);

            self.timers.tick();
            return;
        }

        self.update_difficulty();
        let (enemies_killed, bombs_exploded) = self.entities.update();

        self.update_score(enemies_killed, bombs_exploded);

        if self.entities.killer.is_some() {
            let life = self.entities.players[PlayerN::P1 as usize]
                .as_mut()
                .expect("Player 1 should always exist")
                .entity
                .life
                .borrow_mut();
            *life = life.saturating_sub(1);
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

    #[cfg(debug_assertions)]
    fn print_statistics(&mut self) {
        if self.timers.frame_count % 60 == 0 {
            wasm4::trace(
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
            let life = self.entities.players[PlayerN::P1 as usize]
                .as_mut()
                .expect("Player should exist")
                .entity
                .life
                .borrow_mut();
            *life = life.saturating_add(1);
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
                Bomb::new(&Coord {
                    x: self.calibrations.rng.f64() * (SCREEN_SIZE - 20) as f64 + 10.0,
                    y: self.calibrations.rng.f64() * (SCREEN_SIZE - 20) as f64 + 10.0,
                }),
            );
        }
    }
    fn spawn_enemies(&mut self) {
        // Enemy color depends on time, so we can have nice sections of enemies
        // with same colour, while keeping some element of randomness (their position).
        if self.timers.frame_count % EN_COL_FRAME[self.calibrations.difficulty as usize] == 0 {
            self.calibrations.enemy_color = match self.calibrations.enemy_color {
                DRAW_COLOR_A => DRAW_COLOR_B,
                _ => DRAW_COLOR_A,
            };
        }
        // We only spawn a maximum of 1 enemy per frame, at an interval decided
        // by ENEMY_FRAME. It works fine and even at 1 enemy per frame (60
        // enemies per second) the pressure is high.
        // Enemies are randomly spawned at 8 fixed locations (corners and mid-edges)
        if self.timers.frame_count % ENEMY_FRAME[self.calibrations.difficulty as usize] == 0
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
                Enemy::new(
                    self.timers.frame_count,
                    Coord {
                        x: pos.0 as f64,
                        y: pos.1 as f64,
                    },
                    self.calibrations.enemy_color,
                ),
            );
        }
    }
    fn draw_hud(&self) {
        // Draws score, high-score, multiplier, player life count
        palette::set_draw_color(0x12);
        wasm4::text(self.scores.current.to_string(), 1, 1);
        if self.flags.new_high_score
            && self.flags.current_screen == ScreenName::GameOver
            && (self.timers.frame_count / 2) % 10 < 5
        {
            palette::set_draw_color(0x00);
        }
        wasm4::text(
            "H:".to_string() + self.scores.high.to_string().as_str(),
            73,
            1,
        );
        palette::set_draw_color(0x12);
        wasm4::text(
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

        // Only Player 1 lives count, it owns the whole pool
        for l in 0..self.entities.players[PlayerN::P1 as usize]
            .as_ref()
            .expect("Player should exist")
            .entity
            .life
        {
            wasm4::blit(
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
        // Just shows players and blink killer
        for player in self.entities.players.iter().flatten() {
            player.draw();
        }

        if let Some(killer) = &self.entities.killer {
            if self.timers.death_countdown / 10 % 2 != 0 {
                killer.draw()
            }
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
