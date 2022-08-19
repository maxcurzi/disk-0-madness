use crate::{
    notes::*,
    wasm4::{
        tone, TONE_MODE1, TONE_MODE3, TONE_MODE4, TONE_NOISE, TONE_PAN_LEFT, TONE_PAN_RIGHT,
        TONE_PULSE1, TONE_PULSE2, TONE_TRIANGLE,
    },
};

pub fn music_player(ext_counter: usize, song_n: u8) {
    song_player(&ext_counter, song_n);
}
#[derive(Copy, Clone)]
struct Note {
    note: Option<u32>,
    duration: u32,
}

const VOICE_NOTES: usize = 64;

fn voice_player(counter: &usize, voice: Voice, volume: Volume, flags: Flags) {
    let idx = counter % VOICE_NOTES;
    let note = &voice[idx];
    if let Some(n) = note.note {
        tone(n, note.duration, volume, flags);
    }
}

pub fn song_player(counter: &usize, song_n: u8) {
    let song = SONGS[song_n as usize];
    for voice in song.iter().flatten() {
        voice_player(counter, voice.0, voice.1, voice.2);
    }
}

#[rustfmt::skip]
const GAME_THEME: Voice = [

// Bar 1
Note{note:Some(G4), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 2
Note{note:Some(D5), duration:10},
Note{note:None, duration:0},
Note{note:Some(E5), duration:10},
Note{note:None, duration:0},

// Bar 3
Note{note:Some(D5), duration:10},
Note{note:None, duration:0},
Note{note:Some(E5), duration:10},
Note{note:None, duration:0},

// Bar 4
Note{note:Some(D5), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 5
Note{note:Some(G4), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 6
Note{note:Some(D5), duration:10},
Note{note:None, duration:0},
Note{note:Some(E5), duration:10},
Note{note:None, duration:0},

// Bar 7
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 8
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 9
Note{note:Some(G4), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 10
Note{note:Some(D5), duration:10},
Note{note:None, duration:0},
Note{note:Some(E5), duration:10},
Note{note:None, duration:0},

// Bar 11
Note{note:Some(D5), duration:10},
Note{note:None, duration:0},
Note{note:Some(E5), duration:10},
Note{note:None, duration:0},

// Bar 12
Note{note:Some(D5), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},


// Bar 13
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 14
Note{note:Some(G4), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 15
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 16
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

];

#[rustfmt::skip]
const BEATS0_1: Voice = [

// Bar 1
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 2
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 3
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 4
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 5
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 6
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 7
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 8
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 9
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 10
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 11
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 12
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},


// Bar 13
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 14
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 15
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

// Bar 16
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},
Note{note:Some(G3), duration:3},
Note{note:None, duration:0},

];

#[rustfmt::skip]
const BEATS0_2: Voice = [

// Bar 1
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 2
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 3
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 4
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 5
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 6
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 7
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 8
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 9
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 10
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 11
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 12
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 13
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 14
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 15
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(G1), duration:2},
Note{note:None, duration:0},

// Bar 16
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

];

#[rustfmt::skip]
const T_0_0: Voice = [

// Bar 1
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 2
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 3
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 4
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 5
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 6
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 7
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 8
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 9
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 10
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 11
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 12
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 13
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 14
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 15
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 16
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

];

#[rustfmt::skip]
const T_0_1: Voice = [

// Bar 1
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 2
Note{note:Some(E3), duration:8},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 3
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 4
Note{note:Some(E3), duration:8},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 5
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 6
Note{note:Some(E3), duration:8},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 7
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 8
Note{note:Some(E3), duration:8},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 9
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 10
Note{note:Some(E3), duration:8},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 11
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 12
Note{note:Some(E3), duration:8},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 13
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 14
Note{note:Some(E3), duration:8},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 15
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(D3), duration:8},
Note{note:None, duration:0},

// Bar 16
Note{note:Some(E3), duration:8},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

];

#[rustfmt::skip]
const T_2_0: Voice = [

// Bar 1
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 2
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:Some(A3), duration:10},
Note{note:None, duration:0},

// Bar 2
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:Some(A3), duration:10},
Note{note:None, duration:0},

// Bar 2
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 3
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:Some(A3), duration:10},
Note{note:None, duration:0},

// Bar 6
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:Some(A3), duration:10},
Note{note:None, duration:0},

// Bar 7
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 8
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 9
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 10
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:Some(A3), duration:10},
Note{note:None, duration:0},

// Bar 11
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:Some(A3), duration:10},
Note{note:None, duration:0},

// Bar 12
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 12
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:Some(A3), duration:10},
Note{note:None, duration:0},

// Bar 12
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:Some(A3), duration:10},
Note{note:None, duration:0},

// Bar 13
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 16
Note{note:Some(G2), duration:10},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

];

#[rustfmt::skip]
const T_1_0: Voice = [

// Bar 1
Note{note:Some(G3), duration:10},
Note{note:None, duration:0},
Note{note:Some(D4), duration:10},
Note{note:None, duration:0},

// Bar 2
Note{note:Some(A4), duration:10},
Note{note:None, duration:0},
Note{note:Some(E4), duration:10},
Note{note:None, duration:0},

// Bar 3
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(E3), duration:10},
Note{note:None, duration:0},

// Bar 3
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 4
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},

// Bar 6
Note{note:Some(E3), duration:10},
Note{note:None, duration:0},
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},

// Bar 7
Note{note:Some(E3), duration:10},
Note{note:None, duration:0},
Note{note:Some(E3), duration:10},
Note{note:None, duration:0},

// Bar 8
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 9
Note{note:Some(G3), duration:10},
Note{note:None, duration:0},
Note{note:Some(D4), duration:10},
Note{note:None, duration:0},

// Bar 10
Note{note:Some(A4), duration:10},
Note{note:None, duration:0},
Note{note:Some(E4), duration:10},
Note{note:None, duration:0},

// Bar 11
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:Some(E3), duration:10},
Note{note:None, duration:0},

// Bar 12
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 13
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},

// Bar 13
Note{note:Some(E3), duration:10},
Note{note:None, duration:0},
Note{note:Some(D3), duration:10},
Note{note:None, duration:0},

// Bar 14
Note{note:Some(E3), duration:10},
Note{note:None, duration:0},
Note{note:Some(E3), duration:10},
Note{note:None, duration:0},

// Bar 16
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
];

#[rustfmt::skip]
const T_3_0: Voice = [

// Bar 1
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 2
Note{note:Some(A4), duration:1 | (16 << 8)},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 3
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 4
Note{note:Some(A4), duration:1 | (16 << 8)},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 5
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 6
Note{note:Some(A4), duration:1 | (16 << 8)},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 7
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 8
Note{note:Some(A4), duration:1 | (16 << 8)},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 9
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 10
Note{note:Some(A4), duration:1 | (16 << 8)},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 11
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 12
Note{note:Some(A4), duration:1 | (16 << 8)},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 13
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 14
Note{note:Some(A4), duration:1 | (16 << 8)},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 15
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

// Bar 16
Note{note:Some(A4), duration:1 | (16 << 8)},
Note{note:None, duration:0},
Note{note:None, duration:0},
Note{note:None, duration:0},

];
type Voice = [Note; VOICE_NOTES];
type Volume = u32;
type Flags = u32;
type Track = Option<(Voice, Volume, Flags)>;
type Song = [Track; 4];

const SONG0: Song = [
    Some((GAME_THEME, 40, TONE_PULSE1 | TONE_MODE4)),
    Some((BEATS0_1, 50, TONE_TRIANGLE | TONE_PAN_LEFT)),
    Some((BEATS0_2, 80, TONE_PULSE2 | TONE_PAN_RIGHT)),
    None,
    // None,
];

const SONG1_0: Song = [
    None,
    None,
    Some((T_2_0, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    None,
];
const SONG1_1: Song = [
    Some((T_0_0, 20 | (10 << 8), TONE_PULSE1 | TONE_MODE1)),
    None,
    Some((T_2_0, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    None,
];
const SONG1_2: Song = [
    Some((T_0_1, 20 | (10 << 8), TONE_PULSE1 | TONE_MODE1)),
    None,
    Some((T_2_0, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    None,
];
const SONG1_3: Song = [
    Some((T_0_1, 20 | (10 << 8), TONE_PULSE1 | TONE_MODE1)),
    Some((T_1_0, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    Some((T_2_0, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    None,
];
const SONG1_4: Song = [
    Some((T_0_1, 60 | (10 << 8), TONE_PULSE1 | TONE_MODE1)),
    Some((T_1_0, 80, TONE_PULSE2 | TONE_MODE1)),
    Some((T_2_0, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    Some((T_3_0, 40, TONE_NOISE | TONE_MODE3)),
];
const SONGS: [Song; 2] = [SONG0, SONG1_0]; //, SONG1_1]; //, SONG1_2]; //, SONG1_3]; // SONG1_4];
                                           // const SONGS: [Song; 1] = [SONG0]; //, SONG1_0, SONG1_1, SONG1_2, SONG1_3, SONG1_4];
