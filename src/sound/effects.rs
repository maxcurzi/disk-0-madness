#![allow(unused)]
use super::notes::*;
use crate::wasm4::{
    self, TONE_MODE1, TONE_MODE2, TONE_MODE3, TONE_MODE4, TONE_NOISE, TONE_PAN_LEFT,
    TONE_PAN_RIGHT, TONE_PULSE1, TONE_PULSE2, TONE_TRIANGLE,
};

pub fn bomb_explode() {
    wasm4::tone(
        380 | (10 << 16),
        10 | (10 << 16),
        10,
        TONE_PULSE1 | TONE_MODE3,
    );
}
pub fn death() {
    wasm4::tone(
        140 | (110 << 16),
        3 | (6 << 16),
        60,
        TONE_NOISE | TONE_MODE3,
    );
}

pub fn extra_life() {
    wasm4::tone(
        6000 << 16,
        1 | (3 << 8) | (8 << 16) | (3 << 24),
        100 | (100 << 8),
        TONE_PULSE1 | TONE_MODE1,
    );
}

pub fn new_player() {
    wasm4::tone(400 | 1000 << 16, 10, 100, TONE_PULSE2 | TONE_MODE1);
}
pub fn color1_switch() {
    wasm4::tone(
        340,
        1 | (3 << 8) | (8 << 16),
        24 << 8,
        TONE_TRIANGLE | TONE_MODE1,
    );
}
pub fn color2_switch() {
    wasm4::tone(
        360,
        1 | (3 << 8) | (8 << 16) | (3 << 24),
        24 << 8,
        TONE_TRIANGLE | TONE_MODE1,
    );
}
