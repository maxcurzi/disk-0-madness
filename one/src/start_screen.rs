use crate::{
    bomb::Bomb,
    enemy::Enemy1,
    entity::{Coord, Visible},
    palette::{set_draw_color, COLOR1, COLOR2},
    snake::Snake1,
    wasm4::{rect, text},
};

pub fn start_screen() {
    let voff = 5;
    let hoff = 20;
    set_draw_color(0x11);
    rect(hoff, voff, 120, 140);
    set_draw_color(0x23);
    rect(hoff - 18, voff + 3, 156, 14);
    set_draw_color(0x02);
    text("--- HOW TO PLAY ---", hoff - 16, voff + 7);
    let mut snake1 = Snake1::new();
    let mut snake2 = Snake1::new();
    let mut snake3 = Snake1::new();
    let mut snake4 = Snake1::new();

    set_draw_color(0x12);
    text("   You->", hoff, voff + 25);
    text(" Avoid->", hoff, voff + 35);
    text("Absorb->", hoff, voff + 45);
    snake1.set_position(Coord {
        x: hoff as f64 + 70.0,
        y: voff as f64 + 25.0,
    });
    snake1.draw();

    let enemy = Enemy1::new(0, hoff as f64 + 71.0, voff as f64 + 36.0, COLOR1);
    enemy.draw();
    let enemy = Enemy1::new(0, hoff as f64 + 71.0, voff as f64 + 46.0, COLOR2);
    enemy.draw();

    set_draw_color(0x12);
    text("Push X: -> ->", hoff, voff + 65);
    snake2.set_position(Coord {
        x: hoff as f64 + 55.0,
        y: voff as f64 + 65.0,
    });
    snake2.draw();
    snake3.set_position(Coord {
        x: hoff as f64 + 80.0,
        y: voff as f64 + 65.0,
    });
    snake3.switch_color();
    snake3.draw();
    snake4.set_position(Coord {
        x: hoff as f64 + 105.0,
        y: voff as f64 + 65.0,
    });
    snake4.draw();
    set_draw_color(0x12);
    text("Bombs  change", hoff, voff + 85);
    text("enemy color", hoff, voff + 95);
    text("to your color!", hoff, voff + 105);
    let bomb = Bomb::new(&Coord {
        x: hoff as f64 + 43.0,
        y: voff as f64 + 84.0,
    });
    bomb.draw();

    set_draw_color(0x23);
    rect(hoff - 10, voff + 122, 140, 14);
    set_draw_color(0x04);
    text("Push X to start", hoff, voff + 125);
    set_draw_color(0x13);
    text("Z: palette", hoff + 60, voff + 145);
}
