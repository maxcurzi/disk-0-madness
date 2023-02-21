use crate::graphics::screen::ScreenName;

/// Flags are here used mainly to control which screen needs to be shown and
/// little else. There's probably a simpler way of handling screens and
/// transitions but given the game has a very linear structure the approach
/// works fine.
pub struct Flags {
    pub current_screen: ScreenName,
    pub new_high_score: bool,
}
impl Flags {
    pub fn new() -> Self {
        Self {
            current_screen: ScreenName::Title,
            new_high_score: false,
        }
    }
}
