#![allow(unused)]
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

const VOICE_NOTES: usize = 64;

fn voice_player(counter: &usize, voice: Voice, duration: Duration, volume: Volume, flags: Flags) {
    let idx = counter % VOICE_NOTES;
    let note = voice[idx];
    if note != XX {
        tone(note as u32, duration, volume, flags);
    }
}

pub fn song_player(counter: &usize, song_n: u8) {
    let song = SONGS[song_n as usize];
    for voice in song.iter().flatten() {
        voice_player(counter, voice.0, voice.1, voice.2, voice.3);
    }
}

// Music is as follows:
// - A Song contains up to 4 tracks (PULSE1, PULSE2, TRIANGLE, NOISE).
//
// - Tracks: Each track is a combination of a Voice (sequence of notes) and
// their corresponding volume/duration/flags.
// Duration, Volume, and flags are applied uniformly within each voice
// (this doesn't allow much variety, but I can make it work).
//
// - Voice: Each Voice has EXACTLY 64 notes (16 bars). Wastes space if
// the track is mostly empty, but keeping index:note can be just as wasteful.
//

type Note = u16;
type Voice = [Note; VOICE_NOTES];
type Volume = u32;
type Duration = u32;
type Flags = u32;
type Track = Option<(Voice, Duration, Volume, Flags)>;
type Song = [Track; 4];

const SONG0: Song = [
    Some((GAME_THEME, 10, 40, TONE_PULSE1 | TONE_MODE4)),
    Some((BEATS0_1, 3, 50, TONE_TRIANGLE | TONE_PAN_LEFT)),
    Some((BEATS0_2, 2, 80, TONE_PULSE2 | TONE_PAN_RIGHT)),
    None,
];
const SONG1_0: Song = [
    None,
    None,
    Some((T_2_0, 10, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    None,
];
const SONG1_1: Song = [
    Some((T_0_0, 8, 20 | (10 << 8), TONE_PULSE1 | TONE_MODE1)),
    None,
    Some((T_2_0, 10, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    None,
];
const SONG1_2: Song = [
    Some((T_0_1, 8, 20 | (10 << 8), TONE_PULSE1 | TONE_MODE1)),
    None,
    Some((T_2_0, 10, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    None,
];
const SONG1_3: Song = [
    Some((T_0_1, 8, 40 | (10 << 8), TONE_PULSE1 | TONE_MODE1)),
    None,
    Some((T_2_0, 10, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    Some((T_3_0, 1 | (16 << 8), 40, TONE_NOISE | TONE_MODE3)),
];
const SONG1_4: Song = [
    Some((T_0_1, 8, 60 | (10 << 8), TONE_PULSE1 | TONE_MODE1)),
    Some((T_1_0, 10, 80, TONE_PULSE2 | TONE_MODE1)),
    Some((T_2_0, 10, 60 | (10 << 8), TONE_TRIANGLE | TONE_MODE1)),
    Some((T_3_0, 1 | (16 << 8), 40, TONE_NOISE | TONE_MODE3)),
];
const SONGS: [Song; 6] = [SONG0, SONG1_0, SONG1_1, SONG1_2, SONG1_3, SONG1_4];

#[rustfmt::skip]
const GAME_THEME: Voice = [

// Bar 1
G4, XX, XX, XX,
// Bar 2
D5, XX, E5, XX,
// Bar 3
D5, XX, E5, XX,
// Bar 4
D5, XX, XX, XX,
// Bar 5
G4, XX, XX, XX,
// Bar 6
D5, XX, E5, XX,
// Bar 7
XX, XX, XX, XX,
// Bar 8
XX, XX, XX, XX,
// Bar 9
G4, XX, XX, XX,
// Bar 10
D5, XX, E5, XX,
// Bar 11
D5, XX, E5, XX,
// Bar 12
D5, XX, XX, XX,
// Bar 13
D3, XX, XX, XX,
// Bar 14
G4, XX, XX, XX,
// Bar 15
XX, XX, XX, XX,
// Bar 16
XX, XX, XX, XX,
];

#[rustfmt::skip]
const BEATS0_1: Voice = [

// Bar 1
G3, XX, G3, XX,
// Bar 2
G3, XX, G3, XX,
// Bar 3
G3, XX, G3, XX,
// Bar 4
G3, XX, G3, XX,
// Bar 5
G3, XX, G3, XX,
// Bar 6
G3, XX, G3, XX,
// Bar 7
G3, XX, G3, XX,
// Bar 8
G3, XX, G3, XX,
// Bar 9
G3, XX, G3, XX,
// Bar 10
G3, XX, G3, XX,
// Bar 11
G3, XX, G3, XX,
// Bar 12
G3, XX, G3, XX,
// Bar 13
G3, XX, G3, XX,
// Bar 14
G3, XX, G3, XX,
// Bar 15
G3, XX, G3, XX,
// Bar 16
G3, XX, G3, XX,
];

#[rustfmt::skip]
const BEATS0_2: Voice = [
// Bar 1
XX, XX, G1, XX,
// Bar 2
XX, XX, G1, XX,
// Bar 3
XX, XX, G1, XX,
// Bar 4
XX, XX, G1, XX,
// Bar 5
G1, XX, XX, XX,
// Bar 6
XX, XX, G1, XX,
// Bar 7
XX, XX, G1, XX,
// Bar 8
XX, XX, XX, XX,
// Bar 9
XX, XX, G1, XX,
// Bar 10
XX, XX, G1, XX,
// Bar 11
XX, XX, G1, XX,
// Bar 12
XX, XX, G1, XX,
// Bar 13
G1, XX, XX, XX,
// Bar 14
XX, XX, G1, XX,
// Bar 15
XX, XX, G1, XX,
// Bar 16
XX, XX, XX, XX,
];

#[rustfmt::skip]
const T_0_0: Voice = [

// Bar 1
D3, XX, XX, XX,
// Bar 2
XX, XX, D3, XX,
// Bar 3
XX, XX, D3, XX,
// Bar 4
XX, XX, XX, XX,
// Bar 5
D3, XX, XX, XX,
// Bar 6
XX, XX, D3, XX,
// Bar 7
XX, XX, D3, XX,
// Bar 8
XX, XX, XX, XX,
// Bar 9
D3, XX, XX, XX,
// Bar 10
XX, XX, D3, XX,
// Bar 11
XX, XX, D3, XX,
// Bar 12
XX, XX, XX, XX,
// Bar 13
D3, XX, XX, XX,
// Bar 14
XX, XX, D3, XX,
// Bar 15
XX, XX, D3, XX,
// Bar 16
XX, XX, XX, XX,
];

#[rustfmt::skip]
const T_0_1: Voice = [

// Bar 1
D3, XX, XX, XX,
// Bar 2
E3, XX, D3, XX,
// Bar 3
XX, XX, D3, XX,
// Bar 4
E3, XX, XX, XX,
// Bar 5
D3, XX, XX, XX,
// Bar 6
E3, XX, D3, XX,
// Bar 7
XX, XX, D3, XX,
// Bar 8
E3, XX, XX, XX,
// Bar 9
D3, XX, XX, XX,
// Bar 10
E3, XX, D3, XX,
// Bar 11
XX, XX, D3, XX,
// Bar 12
E3, XX, XX, XX,
// Bar 13
D3, XX, XX, XX,
// Bar 14
E3, XX, D3, XX,
// Bar 15
XX, XX, D3, XX,
// Bar 16
E3, XX, XX, XX,
];

#[rustfmt::skip]
const T_2_0: Voice = [

// Bar 1
XX, XX, XX, XX,
// Bar 2
G2, XX, A3, XX,
// Bar 2
G2, XX, A3, XX,
// Bar 2
G2, XX, XX, XX,
// Bar 3
G2, XX, A3, XX,
// Bar 6
G2, XX, A3, XX,
// Bar 7
G2, XX, XX, XX,
// Bar 8
G2, XX, XX, XX,
// Bar 9
XX, XX, XX, XX,
// Bar 10
G2, XX, A3, XX,
// Bar 11
G2, XX, A3, XX,
// Bar 12
G2, XX, XX, XX,
// Bar 12
G2, XX, A3, XX,
// Bar 12
G2, XX, A3, XX,
// Bar 13
G2, XX, XX, XX,
// Bar 16
G2, XX, XX, XX,
];

#[rustfmt::skip]
const T_1_0: Voice = [

// Bar 1
G3, XX, D4, XX,
// Bar 2
A4, XX, E4, XX,
// Bar 3
XX, XX, E3, XX,
// Bar 3
XX, XX, XX, XX,
// Bar 4
D3, XX, D3, XX,
// Bar 6
E3, XX, D3, XX,
// Bar 7
E3, XX, E3, XX,
// Bar 8
XX, XX, XX, XX,
// Bar 9
G3, XX, D4, XX,
// Bar 10
A4, XX, E4, XX,
// Bar 11
XX, XX, E3, XX,
// Bar 12
XX, XX, XX, XX,
// Bar 13
D3, XX, D3, XX,
// Bar 13
E3, XX, D3, XX,
// Bar 14
E3, XX, E3, XX,
// Bar 16
XX, XX, XX, XX,
];

#[rustfmt::skip]
const T_3_0: Voice = [

// Bar 1
XX, XX, XX, XX,
// Bar 2
A4, XX, XX, XX,
// Bar 3
XX, XX, XX, XX,
// Bar 4
A4, XX, XX, XX,
// Bar 5
XX, XX, XX, XX,
// Bar 6
A4, XX, XX, XX,
// Bar 7
XX, XX, XX, XX,
// Bar 8
A4, XX, XX, XX,
// Bar 9
XX, XX, XX, XX,
// Bar 10
A4, XX, XX, XX,
// Bar 11
XX, XX, XX, XX,
// Bar 12
A4, XX, XX, XX,
// Bar 13
XX, XX, XX, XX,
// Bar 14
A4, XX, XX, XX,
// Bar 15
XX, XX, XX, XX,
// Bar 16
A4, XX, XX, XX,
];
