mod controls;
mod environment;
mod flags;
mod scores;
mod timers;

use controls::{ControlEvent, Controls};
use environment::Environment;
use flags::Flags;
use scores::Scores;
use timers::Timers;

use crate::{
    common::calibrations::{
        Calibrations, BOMB_FRAME_FREQ, DEATH_COUNTDOWN_DURATION, DIFF_MUL_PROGRESSION, ENEMY_FRAME,
        EN_COL_FRAME, INIT_LIVES, MAX_BOMBS, MAX_ENEMIES, MUSIC_SPEED_CTRL, RESPITE_DURATION,
    },
    common::types::Coord,
    entities::{
        bomb::Bomb,
        enemy::Enemy,
        manager::EntityManager as Entities,
        player::{Player, PlayerN},
        traits::Visible,
    },
    graphics::{
        draw_utils::{self},
        palette::{DRAW_COLOR_A, DRAW_COLOR_B},
        screen::{self, ScreenName},
    },
    sound::{
        effects,
        music::{GAME_OVER_SONG, GAME_SONG_START, VOICE_NOTES},
    },
    wasm4::{self, SCREEN_SIZE},
};
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

// Ideally it should be refactored quite a bit, maybe with the addition of an
// event system.
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
            .expect("P1 should always exist")
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
                    if movement_enabled {
                        if let Some(player) = self.entities.players[player_n as usize].as_mut() {
                            player.left();
                        }
                    }
                }
                ControlEvent::Down(player_n) => {
                    if movement_enabled {
                        if let Some(player) = self.entities.players[player_n as usize].as_mut() {
                            player.down();
                        }
                    }
                }
                ControlEvent::Up(player_n) => {
                    if movement_enabled {
                        if let Some(player) = self.entities.players[player_n as usize].as_mut() {
                            player.up();
                        }
                    }
                }
                ControlEvent::Right(player_n) => {
                    if movement_enabled {
                        if let Some(player) = self.entities.players[player_n as usize].as_mut() {
                            player.right();
                        }
                    }
                }
                ControlEvent::Btn1(player_n) => {
                    if movement_enabled {
                        if let Some(player) = self.entities.players[player_n as usize].as_mut() {
                            player.toggle_color();
                        }
                    }
                    // New player joins!
                    if self.entities.players[player_n as usize].is_none() {
                        wasm4::trace(
                            "Player ".to_owned()
                                + (player_n as u8 + 1).to_string().as_str()
                                + " joined!",
                        );
                        self.entities.players[player_n as usize] = Some(Player::new(player_n));
                        if let Some(player) = self.entities.players[PlayerN::P1 as usize].as_mut() {
                            player.entity.life += INIT_LIVES;
                        }
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
    /// self.timers.tick() exactly once every time it's called. It's called at
    /// 60fps and it returns early depending on the current screen.
    pub fn update(&mut self) {
        self.timers.tick();
        self.environment
            .update(self.timers.frame_count, self.timers.song_tick);
        self.process_inputs();

        // First we show the title screen
        if self.flags.current_screen == ScreenName::Title {
            screen::title(self.timers.frame_count);
            return;
        }
        // Then how to play
        if self.flags.current_screen == ScreenName::HowToPlay {
            screen::how_to_play(self.timers.frame_count);
            return;
        }

        // When the game starts the HUD will be always visible
        draw_utils::draw_hud(
            self.entities.players[PlayerN::P1 as usize]
                .as_ref()
                .expect("P1 should always exist")
                .entity
                .life,
            self.scores.current,
            self.scores.high,
            self.scores.multiplier,
            self.flags.current_screen != ScreenName::GameOver
                || !self.flags.new_high_score
                || self.flags.current_screen == ScreenName::GameOver
                    && (self.timers.frame_count / 2) % 10 < 5,
        );

        // Game over screen
        if self.flags.current_screen == ScreenName::GameOver {
            screen::game_over(self.timers.frame_count);
            return;
        }

        // Stop-the-world death event
        if self.entities.killer.is_some() {
            self.death_tick();
            return;
        }

        // End-game. Player ran out of lives, save high score and flag for game-over
        if self.entities.players[PlayerN::P1 as usize]
            .as_ref()
            .expect("P1 should always exist")
            .entity
            .life
            == 0
        {
            self.flags.new_high_score = self.scores.current > self.scores.high;
            self.scores.high = std::cmp::max(self.scores.current, self.scores.high);
            self.flags.current_screen = ScreenName::GameOver;
            self.environment.song_nr = GAME_OVER_SONG;
            self.timers.song_tick = 0;

            // Save high score
            let high_score: u32 = self.scores.high;
            self.scores.save_high_score(high_score);
            return;
        }

        self.update_difficulty();
        let (enemies_killed, bombs_exploded) = self.entities.update();

        self.update_score(enemies_killed, bombs_exploded);

        if self.entities.killer.is_some() {
            if let Some(player) = self.entities.players[PlayerN::P1 as usize].as_mut() {
                player.entity.life = player.entity.life.saturating_sub(1);
            }
        }

        self.spawn_enemies();
        self.spawn_bombs();
        self.entities.draw();

        self.sounds_and_music_tick(bombs_exploded);

        // Print Statistics
        #[cfg(debug_assertions)]
        self.print_statistics();
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
            if let Some(player) = self.entities.players[PlayerN::P1 as usize].as_mut() {
                player.entity.life = player.entity.life.saturating_add(1);
            }
            self.calibrations.score_next_life = self.calibrations.score_next_life.saturating_mul(2);
        }
    }

    fn update_difficulty(&mut self) {
        // Set difficulty. It simply depends on the current multiplier.
        for (i, mul) in DIFF_MUL_PROGRESSION.iter().enumerate() {
            if self.scores.multiplier < *mul {
                self.calibrations.difficulty =
                    std::cmp::max(i as u32, self.calibrations.difficulty);
                break;
            }
        }
    }

    fn spawn_bombs(&mut self) {
        // Bombs are spawned with a similar logic to the enemies, but in random positions on screen.
        if self.timers.frame_count % BOMB_FRAME_FREQ == 0 && self.entities.bombs.len() < MAX_BOMBS {
            self.entities.bombs.insert(
                self.timers.frame_count,
                Box::new(Bomb::new(&Coord {
                    x: self.calibrations.rng.f64() * (SCREEN_SIZE - 20) as f64 + 10.0,
                    y: self.calibrations.rng.f64() * (SCREEN_SIZE - 20) as f64 + 10.0,
                })),
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
                Box::new(Enemy::new(
                    self.timers.frame_count,
                    Coord {
                        x: pos.0 as f64,
                        y: pos.1 as f64,
                    },
                    self.calibrations.enemy_color,
                )),
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
