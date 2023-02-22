use crate::common::calibrations::{DEATH_COUNTDOWN_DURATION, RESPITE_DURATION};

/// Counters to keep track of time in music and some events
pub struct Timers {
    // The main game counter is frame_count (the frame counter) used as "tick"
    // in many operations, some timers are helpful to control the duration of
    // the death animation, or the time without enemies when the player spawns.
    //
    // Song tick is mainly used to start the "game" and "game_over" songs on the
    // first beat. The other main game song is composed of multiple "songs"
    // which are switched depending on the level, but it should be perceived as
    // a single song without beat changes. The synchronization is  handled in
    // the song player
    pub frame_count: usize,
    pub death_countdown: usize,
    pub respite: usize, // frames without enemies
    pub song_tick: usize,
}
impl Timers {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            death_countdown: DEATH_COUNTDOWN_DURATION,
            respite: RESPITE_DURATION,
            song_tick: 0,
        }
    }

    pub fn tick(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);
        self.respite = self.respite.saturating_sub(1);
        self.song_tick = self.song_tick.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update() {
        let mut timers = Timers::new();
        timers.tick();
        assert_eq!(timers.frame_count, 1);
        assert_eq!(timers.respite, RESPITE_DURATION - 1);
        assert_eq!(timers.song_tick, 1);

        timers.tick();
        assert_eq!(timers.frame_count, 2);
        assert_eq!(timers.respite, RESPITE_DURATION - 2);
        assert_eq!(timers.song_tick, 2);
    }
}
