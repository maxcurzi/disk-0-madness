use crate::{
    entities::player::PlayerN,
    wasm4::{
        BUTTON_1, BUTTON_2, BUTTON_DOWN, BUTTON_LEFT, BUTTON_RIGHT, BUTTON_UP, GAMEPAD1, GAMEPAD2,
        GAMEPAD3, GAMEPAD4, MOUSE_BUTTONS, MOUSE_LEFT, MOUSE_MIDDLE, MOUSE_RIGHT, MOUSE_X, MOUSE_Y,
        SCREEN_SIZE,
    },
};

pub enum ControlEvent {
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
pub struct Controls {
    pub prev_mouse: u8,
    pub prev_gamepad1: u8,
    pub prev_gamepad2: u8,
    pub prev_gamepad3: u8,
    pub prev_gamepad4: u8,
}
impl Controls {
    const MOUSE_AREA_PADDING: i16 = 20; // Extra space around play area to allow mouse events.
    pub fn new() -> Self {
        Self {
            prev_mouse: unsafe { *MOUSE_BUTTONS },
            prev_gamepad1: unsafe { *GAMEPAD1 },
            prev_gamepad2: unsafe { *GAMEPAD2 },
            prev_gamepad3: unsafe { *GAMEPAD3 },
            prev_gamepad4: unsafe { *GAMEPAD4 },
        }
    }

    /// Read from peripherals and return everything that's happening
    pub fn update(&mut self) -> Vec<ControlEvent> {
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
