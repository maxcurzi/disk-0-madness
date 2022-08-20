use crate::{
    bomb::Bomb,
    enemy::Enemy1,
    entity::{Coord, Visible},
    intro_screen::{INTRO_SCREEN, INTRO_SCREEN_FLAGS, INTRO_SCREEN_HEIGHT, INTRO_SCREEN_WIDTH},
    palette::{set_draw_color, COLOR1, COLOR2},
    snake::Snake1,
    title_image::{TITLE1, TITLE1_FLAGS, TITLE1_HEIGHT, TITLE1_WIDTH},
    wasm4::{blit, rect, text, DRAW_COLORS, SCREEN_SIZE},
};

const X_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x80]) };
const Z_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x81]) };
const LEFT_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x84]) };
const RIGHT_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x85]) };
const UP_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x86]) };
const DOWN_ICON: &str = unsafe { std::str::from_utf8_unchecked(&[0x87]) };

pub fn title_screen(tick: usize) {
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

pub fn htp_screen(tick: usize) {
    let voff = 5;
    let hoff = 20;

    // set_draw_color(0x13);
    // text(x, 1, 1);
    set_draw_color(0x11);
    rect(hoff, voff, 120, 140);
    set_draw_color(0x23);
    rect(hoff - 18, voff + 3, 156, 15);
    set_draw_color(0x02);
    text("--- HOW TO PLAY ---", hoff - 16, voff + 7);
    let mut snake1 = Snake1::new();

    set_draw_color(0x12);
    text("   You:", hoff, voff + 25);
    text(" Avoid:", hoff, voff + 35);
    text("Absorb:", hoff, voff + 45);
    snake1.set_position(Coord {
        x: hoff as f64 + 59.0,
        y: voff as f64 + 25.0,
    });
    snake1.draw();

    let enemy = Enemy1::new(0, hoff as f64 + 60.0, voff as f64 + 36.0, COLOR1);
    enemy.draw();
    let enemy = Enemy1::new(0, hoff as f64 + 60.0, voff as f64 + 46.0, COLOR2);
    enemy.draw();

    set_draw_color(0x12);
    // let X = "";
    // let Z = "";
    // let left = "";
    // let right = "";
    // let up = "";
    // let down = "";
    // text(X.to_string(), 10, 10);
    // text(Z.to_string(), 10, 20);
    // text(right, hoff + 60, voff + 55);
    // text(down, hoff + 51, voff + 55);
    // text(up, hoff + 42, voff + 55);
    // text(left, hoff + 33, voff + 55);
    // text(" ", hoff+ 33, 20);

    text(
        LEFT_ICON.to_owned() + DOWN_ICON + UP_ICON + RIGHT_ICON + ":Move",
        hoff,
        voff + 60,
    );
    // text("/Mouse", hoff + 76, voff + 55);
    set_draw_color(0x12);
    text("   ".to_owned() + X_ICON + ": -> ->", hoff, voff + 70);
    // text(X, hoff + 32, voff + 65);
    // text(" ", hoff + 32, voff + 65);
    snake1.set_position(Coord {
        x: hoff as f64 + 40.0,
        y: voff as f64 + 70.0,
    });
    snake1.draw();
    snake1.set_position(Coord {
        x: hoff as f64 + 64.0,
        y: voff as f64 + 70.0,
    });
    snake1.switch_color();
    snake1.draw();
    snake1.switch_color();
    snake1.set_position(Coord {
        x: hoff as f64 + 88.0,
        y: voff as f64 + 70.0,
    });
    snake1.draw();
    set_draw_color(0x12);
    text("Bombs  change", hoff, voff + 85);
    text("the enemy color", hoff, voff + 95);
    text("to your color!", hoff, voff + 105);
    let bomb = Bomb::new(&Coord {
        x: hoff as f64 + 43.0,
        y: voff as f64 + 84.0,
    });
    bomb.draw();

    set_draw_color(0x23);
    rect(hoff - 10, voff + 122, 140, 13);
    if (tick / 5) % 10 < 4 {
        set_draw_color(0x00);
    } else {
        set_draw_color(0x04);
    }
    text(
        "Press ".to_owned() + X_ICON + " to start",
        hoff - 4,
        voff + 125,
    );
    set_draw_color(0x13);
    text(Z_ICON.to_owned() + ":palette", hoff + 66, voff + 145);
}

pub fn game_over_screen(tick: usize) {
    set_draw_color(0x14);
    text(
        "GAME OVER",
        SCREEN_SIZE as i32 / 2 - 35,
        SCREEN_SIZE as i32 / 2 - 10,
    );
    if (tick / 5) % 10 < 4 {
        set_draw_color(0x00);
    }
    text(
        "Press ".to_owned() + X_ICON + " to restart",
        8,
        SCREEN_SIZE as i32 / 2 + 13,
    );
}