use crate::{
    bomb::Bomb,
    common::{Coord, Movable, Visible},
    draws,
    enemy::Enemy,
    palette::{self, DRAW_COLOR_A, DRAW_COLOR_B, HEART},
    player::{Player, PlayerN},
    screen::{self, ScreenName},
    sound::{self, GAME_OVER_SONG, GAME_SONG_START, INTRO_SONG, VOICE_NOTES},
    wasm4::{
        self, BLIT_1BPP, BUTTON_1, BUTTON_2, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP,
        GAMEPAD1, GAMEPAD2, GAMEPAD3, GAMEPAD4, MOUSE_BUTTONS, MOUSE_LEFT, MOUSE_MIDDLE,
        MOUSE_RIGHT, MOUSE_X, MOUSE_Y, SCREEN_SIZE,
    },
};
use fastrand::Rng;
use std::{cmp::max, collections::HashMap};
const RNG_SEED: u64 = 555;
const MAX_ENEMIES: usize = 200;
const MAX_BOMBS: usize = 16;
const INIT_LIVES: usize = 3;
const INIT_DIFFICULTY: u32 = 0;
const BOMB_FRAME_FREQ: usize = 300;
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
    [12, 30, 80, 120, 240, 320, 450, 1000, 2000];

/// Entities include the player, enemies and bombs
struct Entities {
    players: [Option<Player>; 4],
    bombs: HashMap<usize, Bomb>,
    enemies: HashMap<usize, Enemy>,
    killer: Option<Enemy>,
}
impl Entities {
    fn new() -> Self {
        let mut player = Player::new(PlayerN::P1);
        player.entity.life = INIT_LIVES as u32;
        Self {
            players: [Some(player), None, None, None],
            bombs: HashMap::new(),   //(MAX_BOMBS),
            enemies: HashMap::new(), //(MAX_ENEMIES),
            killer: None,
        }
    }
    fn update(&mut self) -> (u32, u32) {
        self.update_state();

        let (enemies_killed, bombs_exploded) = self.process_collisions();
        (enemies_killed, bombs_exploded)
    }

    fn update_state(&mut self) {
        // Update players position
        for player in self.players.iter_mut().flatten() {
            player.update_position();
            player.stop();
        }

        // Update enemy position
        for enemy in self.enemies.values_mut() {
            let mut to_follow = &self.players[PlayerN::P1 as usize];

            let mut temp_distance = (SCREEN_SIZE * 2) as f64;
            for player in self.players.iter() {
                if let Some(p) = player {
                    let distance = enemy.entity.distance(&p.entity);
                    if distance < temp_distance {
                        temp_distance = distance;
                        to_follow = player;
                    };
                }
            }
            enemy.follow(to_follow);
            enemy.update_position();
        }

        // Update bombs
        for bomb in self.bombs.values_mut() {
            bomb.update();
        }

        // Cleanup dead entities
        self.prune();
    }

    fn prune(&mut self) {
        self.enemies.retain(|_, enemy| enemy.entity.life > 0);
        self.bombs.retain(|_, bomb| bomb.entity.life > 0);
    }

    fn draw(&self) {
        // Draw players before enemies so in case they overlap it would look more
        // "squishy" when escaping. Bombs should cover enemies so are drawn last.
        for p in self.players.iter().flatten() {
            p.draw();
        }
        for e in self.enemies.values() {
            e.draw();
        }
        for b in self.bombs.values() {
            b.draw();
        }
    }

    fn process_collisions(&mut self) -> (u32, u32) {
        // Returned variables
        let mut enemies_killed = 0;
        let mut bombs_exploded = 0;

        // Player-Bomb collision
        'bombs_loop: for bomb in self.bombs.values_mut() {
            for player in self.players.iter().flatten() {
                let extra_reach = 2.0; // Makes bombs easier to trigger
                if bomb.entity.collided_with(&player.entity, extra_reach) && !bomb.exploded {
                    bombs_exploded += 1;
                    bomb.exploded = true;
                    bomb.who_exploded = Some(player.player_number);
                    continue 'bombs_loop;
                }
            }
        }

        'enemies_loop: for enemy in self.enemies.values_mut() {
            // Bomb-Enemy collision, convert enemies to player color
            'bombs_loop: for bomb in self.bombs.values_mut() {
                let extra_reach = 2.0; // Makes enemies easier to convert
                if bomb.exploded && bomb.entity.collided_with(&enemy.entity, extra_reach) {
                    enemy.entity.color = self.players[bomb
                        .who_exploded
                        .expect("Exploded bomb should have a 'owner'")
                        as usize]
                        .as_ref()
                        .expect("Player should still be alive")
                        .entity
                        .color;
                    break 'bombs_loop; // One exploded bomb <=> One player
                }
            }

            // Enemy-Player collision (same color)
            'players_loop: for player in self.players.iter().flatten() {
                if enemy.entity.color == player.entity.color
                    && enemy.entity.collided_with(&player.entity, 2.0)
                {
                    enemy.kill();
                    enemies_killed += 1;
                    break 'players_loop; // Only one player should be able to "eat" one enemy
                }
            }

            // Enemy-Player collision (different colors)
            #[allow(unused_labels)]
            'players_loop: for player in self.players.iter().flatten() {
                if enemy.entity.color != player.entity.color
                    && !enemy.just_spawned()
                    && enemy.entity.collided_with(&player.entity, -2.0)
                {
                    // Player dies
                    self.killer = Some(enemy.clone());
                    enemy.kill();
                    break 'enemies_loop; // Stop everything.
                }
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
            enemy_color: DRAW_COLOR_B,
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
        sound::play_music(song_tick / MUSIC_SPEED_CTRL, self.song_nr);
        self.draw_space();
    }

    fn play_sound_effects(&self, bombs_exploded: bool, extra_life: bool, player_died: bool) {
        // We just have very few sound effects.
        if bombs_exploded {
            sound::bomb_sound();
        }
        if extra_life {
            sound::extra_life_sound();
        }
        if player_died {
            sound::death_sound();
        }
    }

    fn set_palette(&mut self, palette_nr: u8) {
        self.palette_n = palette_nr % palette::PALETTES.len() as u8;
        palette::set_palette_n(self.palette_n as usize)
    }
}

enum ControlEvent {
    MouseLeftHold((i16, i16)),
    MouseLeftClick,
    MouseRightClick,
    MouseMiddleClick,
    Left(PlayerN),
    Down(PlayerN),
    Up(PlayerN),
    Right(PlayerN),
    Btn1(PlayerN),
    Btn2(PlayerN),
}
/// Handles user actions (mainly keyboard and mouse actions)
struct Controls {
    prev_mouse: u8,
    prev_gamepad1: u8,
    prev_gamepad2: u8,
    prev_gamepad3: u8,
    prev_gamepad4: u8,
}
impl Controls {
    const MOUSE_AREA_PADDING: i16 = 20; // Extra space around play area to allow mouse events.
    fn new() -> Self {
        Self {
            prev_mouse: unsafe { *MOUSE_BUTTONS },
            prev_gamepad1: unsafe { *GAMEPAD1 },
            prev_gamepad2: unsafe { *GAMEPAD2 },
            prev_gamepad3: unsafe { *GAMEPAD3 },
            prev_gamepad4: unsafe { *GAMEPAD4 },
        }
    }

    /// Read from peripherals and return everything that's happening
    fn update(&mut self) -> Vec<ControlEvent> {
        // Return value
        let mut event = vec![];

        // Local vars
        let mouse = unsafe { *MOUSE_BUTTONS };
        let just_pressed_mouse = mouse & (mouse ^ self.prev_mouse);

        let gamepad1 = unsafe { *GAMEPAD1 };
        let gamepad2 = unsafe { *GAMEPAD2 };
        let gamepad3 = unsafe { *GAMEPAD3 };
        let gamepad4 = unsafe { *GAMEPAD4 };

        let just_pressed_gamepad1 = gamepad1 & (gamepad1 ^ self.prev_gamepad1);
        let just_pressed_gamepad2 = gamepad2 & (gamepad2 ^ self.prev_gamepad2);
        let just_pressed_gamepad3 = gamepad3 & (gamepad3 ^ self.prev_gamepad3);
        let just_pressed_gamepad4 = gamepad4 & (gamepad4 ^ self.prev_gamepad4);

        // Check mouse
        if mouse & MOUSE_LEFT != 0
            && self.mouse_in_play_area_within_padding(Self::MOUSE_AREA_PADDING)
        {
            let mouse_pos = self.get_mouse_pos();
            event.push(ControlEvent::MouseLeftHold(mouse_pos));
        }
        if just_pressed_mouse & MOUSE_RIGHT != 0
            && self.mouse_in_play_area_within_padding(Self::MOUSE_AREA_PADDING)
        {
            event.push(ControlEvent::MouseRightClick);
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

        // Check gamepads
        for (gamepad, just_pressed, player_n) in [
            (gamepad1, just_pressed_gamepad1, PlayerN::P1),
            (gamepad2, just_pressed_gamepad2, PlayerN::P2),
            (gamepad3, just_pressed_gamepad3, PlayerN::P3),
            (gamepad4, just_pressed_gamepad4, PlayerN::P4),
        ] {
            if gamepad & BUTTON_LEFT != 0 {
                event.push(ControlEvent::Left(player_n));
            }
            if gamepad & BUTTON_DOWN != 0 {
                event.push(ControlEvent::Down(player_n));
            }
            if gamepad & BUTTON_UP != 0 {
                event.push(ControlEvent::Up(player_n));
            }
            if gamepad & BUTTON_RIGHT != 0 {
                event.push(ControlEvent::Right(player_n));
            }
            if just_pressed & BUTTON_1 != 0 {
                event.push(ControlEvent::Btn1(player_n));
            }
            if just_pressed & BUTTON_2 != 0 {
                event.push(ControlEvent::Btn2(player_n));
            }
        }

        self.prev_gamepad1 = gamepad1;
        self.prev_gamepad2 = gamepad2;
        self.prev_gamepad3 = gamepad3;
        self.prev_gamepad4 = gamepad4;
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
        let mouse_pos = self.get_mouse_pos();
        mouse_pos.0 >= -padding
            && mouse_pos.0 <= SCREEN_SIZE as i16 + padding
            && mouse_pos.1 >= -padding
            && mouse_pos.1 <= SCREEN_SIZE as i16 + padding
    }

    fn get_mouse_pos(&self) -> (i16, i16) {
        unsafe { (*MOUSE_X, *MOUSE_Y) }
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
        // let players = self.entities.players;
        self.entities = Entities::new();
        self.calibrations = Calibrations::new(self.timers.frame_count);
        self.environment = Environment::new(&self.calibrations.rng);
        self.timers = Timers::new();
        self.scores = Scores::new();
        self.flags = Flags::new();
        self.flags.current_screen = ScreenName::MainGame;
        if self.entities.players[PlayerN::P1 as usize].is_some() {
            self.entities.players[PlayerN::P1 as usize]
                .as_mut()
                .expect("Player should exist")
                .entity
                .life = INIT_LIVES as u32;
        }
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
                            .as_ref()
                            .expect("Player 1 should always exist")
                            .entity
                            .life;
                        self.entities.players[PlayerN::P1 as usize]
                            .as_mut()
                            .expect("Player 1 should always exist")
                            .entity
                            .life = lives + INIT_LIVES as u32;
                        sound::new_player();
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
                .as_ref()
                .expect("Player 1 should always exist")
                .entity
                .life;

            self.entities.players[PlayerN::P1 as usize]
                .as_mut()
                .expect("Player 1 should always exist")
                .entity
                .life = life.saturating_sub(1);
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
                .as_ref()
                .expect("Player should exist")
                .entity
                .life;
            self.entities.players[PlayerN::P1 as usize]
                .as_mut()
                .expect("Player should exist")
                .entity
                .life = life.saturating_add(1);
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
                DRAW_COLOR_B => DRAW_COLOR_A,
                _ => panic!("Invalid enemy color"),
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
        // Just shows player and blink killer
        for player in self.entities.players.as_ref() {
            if player.is_some() {
                player.as_ref().expect("Player should exist").draw();
            }
        }

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
