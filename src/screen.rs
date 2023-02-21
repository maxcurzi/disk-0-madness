use crate::{
    bomb::Bomb,
    common::{Coord, Visible},
    enemy::Enemy,
    intro_screen::{INTRO_SCREEN, INTRO_SCREEN_FLAGS, INTRO_SCREEN_HEIGHT, INTRO_SCREEN_WIDTH},
    palette::{set_draw_color, COLOR1, COLOR2},
    player::{Player, PlayerN},
    title_image::{TITLE1, TITLE1_FLAGS, TITLE1_HEIGHT, TITLE1_WIDTH},
    wasm4::{blit, rect, text, DRAW_COLORS, SCREEN_SIZE},
};

const X_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x80]) };
const Z_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x81]) };
const LEFT_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x84]) };
const RIGHT_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x85]) };
const UP_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x86]) };
const DOWN_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x87]) };

// Lmouse_icon
const LMOUSE_ICON_WIDTH: u32 = 8;
const LMOUSE_ICON_HEIGHT: u32 = 8;
const LMOUSE_ICON_FLAGS: u32 = 1; // BLIT_2BPP
const LMOUSE_ICON: [u8; 16] = [
    0xff, 0xff, 0xfa, 0xab, 0xe0, 0x96, 0xe0, 0x96, 0xea, 0xaa, 0xe5, 0x56, 0xf9, 0x5b, 0xfe, 0xaf,
];

// Rmouse_icon
const RMOUSE_ICON_WIDTH: u32 = 8;
const RMOUSE_ICON_HEIGHT: u32 = 8;
const RMOUSE_ICON_FLAGS: u32 = 1; // BLIT_2BPP
const RMOUSE_ICON: [u8; 16] = [
    0xea, 0xaf, 0x96, 0x0b, 0x96, 0x0b, 0xaa, 0xab, 0x95, 0x5b, 0xe5, 0x6f, 0xfa, 0xbf, 0xff, 0xff,
];

// Cmouse_icon
const CMOUSE_ICON_WIDTH: u32 = 8;
const CMOUSE_ICON_HEIGHT: u32 = 8;
const CMOUSE_ICON_FLAGS: u32 = 1; // BLIT_2BPP
const CMOUSE_ICON: [u8; 16] = [
    0xea, 0xaf, 0x94, 0x5b, 0x94, 0x5b, 0xaa, 0xab, 0x95, 0x5b, 0xe5, 0x6f, 0xfa, 0xbf, 0xff, 0xff,
];

#[derive(PartialEq, Eq)]
pub enum ScreenName {
    Title,
    HowToPlay,
    MainGame,
    GameOver,
}

pub fn title(tick: usize) {
    unsafe { *DRAW_COLORS = 0x1234 };
    blit(
        &INTRO_SCREEN,
        0,
        0,
        INTRO_SCREEN_WIDTH,
        INTRO_SCREEN_HEIGHT,
        INTRO_SCREEN_FLAGS,
    );
    unsafe { *DRAW_COLORS = 0x0234 };
    blit(
        &TITLE1,
        20 + tick as i32 % 11 / 4,
        110 + tick as i32 % 7 / 4,
        TITLE1_WIDTH,
        TITLE1_HEIGHT,
        TITLE1_FLAGS,
    );
    set_draw_color(0x02);
}

pub fn how_to_play(tick: usize) {
    const HTP_TEXT_COLOR: u16 = 0x12;
    const HTP_TEXT_COLOR_ALT: u16 = 0x13;
    let voff = 5;
    let hoff = 20;

    set_draw_color(0x11);
    rect(hoff, voff, SCREEN_SIZE - 40, SCREEN_SIZE - 20);
    set_draw_color(0x23);
    rect(hoff - 18, voff + 3, SCREEN_SIZE - 4, 15);
    set_draw_color(0x02);
    text("--- HOW TO PLAY ---", hoff - 16, voff + 7);
    let mut player = Player::new(PlayerN::P1);

    set_draw_color(HTP_TEXT_COLOR);
    text("   You:", hoff, voff + 25);
    text(" Avoid:", hoff, voff + 35);
    text("Absorb:", hoff, voff + 45);
    text("  Bomb:", hoff, voff + 55);
    player.entity.position = Coord {
        x: hoff as f64 + 59.0,
        y: voff as f64 + 25.0,
    };
    player.draw();

    let enemy = Enemy::new(
        0,
        Coord {
            x: hoff as f64 + 60.0,
            y: voff as f64 + 36.0,
        },
        COLOR1,
    );
    enemy.draw();
    let enemy = Enemy::new(
        0,
        Coord {
            x: hoff as f64 + 60.0,
            y: voff as f64 + 46.0,
        },
        COLOR2,
    );
    enemy.draw();
    let bomb = Bomb::new(&Coord {
        x: hoff as f64 + 58.0,
        y: voff as f64 + 54.0,
    });
    bomb.draw();

    set_draw_color(HTP_TEXT_COLOR);
    text(
        " /".to_owned() + LEFT_ICON + DOWN_ICON + UP_ICON + RIGHT_ICON + ":Move",
        hoff,
        voff + 70,
    );
    unsafe { *DRAW_COLORS = 0x0234 };
    blit(
        &LMOUSE_ICON,
        hoff - 1,
        voff + 69,
        LMOUSE_ICON_WIDTH,
        LMOUSE_ICON_HEIGHT,
        LMOUSE_ICON_FLAGS,
    );
    set_draw_color(HTP_TEXT_COLOR);
    text("    /".to_owned() + X_ICON + ": -> ->", hoff, voff + 80);
    unsafe { *DRAW_COLORS = 0x0234 };
    blit(
        &RMOUSE_ICON,
        hoff + 24,
        voff + 80,
        RMOUSE_ICON_WIDTH,
        RMOUSE_ICON_HEIGHT,
        RMOUSE_ICON_FLAGS,
    );
    player.entity.position = Coord {
        x: hoff as f64 + 55.0,
        y: voff as f64 + 80.0,
    };
    player.draw();
    player.entity.position = Coord {
        x: hoff as f64 + 80.0,
        y: voff as f64 + 80.0,
    };
    player.toggle_color();
    player.draw();
    player.toggle_color();
    player.entity.position = Coord {
        x: hoff as f64 + 104.0,
        y: voff as f64 + 80.0,
    };
    player.draw();

    // set_draw_color(0x12);
    // text("Bombs  change", hoff, voff + 85);
    // text("the enemy color", hoff, voff + 95);
    // text("to your color!", hoff, voff + 105);
    // let bomb = Bomb::new(&Coord {
    //     x: hoff as f64 + 43.0,
    //     y: voff as f64 + 84.0,
    // });
    // bomb.draw();
    set_draw_color(HTP_TEXT_COLOR_ALT);
    text("--Multiplayer--\nUp to 4 Players", hoff, voff + 96);

    set_draw_color(0x23);
    rect(hoff - 10, voff + 122, SCREEN_SIZE - 20, 13);

    // Syncs blink with intro song beat
    if (tick / 4) % 10 < 4 {
        set_draw_color(0x00);
    } else {
        set_draw_color(0x04);
    }
    text(
        "Press ".to_owned() + X_ICON + " to start",
        hoff - 4,
        voff + 125,
    );

    set_draw_color(HTP_TEXT_COLOR_ALT);
    text("/".to_owned() + Z_ICON + ":palette", hoff + 54, voff + 145);
    unsafe { *DRAW_COLORS = 0x0234 };
    blit(
        &CMOUSE_ICON,
        hoff + 46,
        voff + 145,
        CMOUSE_ICON_WIDTH,
        CMOUSE_ICON_HEIGHT,
        CMOUSE_ICON_FLAGS,
    );
}

pub fn game_over(tick: usize) {
    set_draw_color(0x14);
    text(
        "GAME OVER",
        SCREEN_SIZE as i32 / 2 - 35,
        SCREEN_SIZE as i32 / 2 - 10,
    );
    if (tick / 2) % 10 < 5 {
        set_draw_color(0x10);
    }
    text(
        "Press ".to_owned() + X_ICON + " to restart",
        8,
        SCREEN_SIZE as i32 / 2 + 13,
    );
}
