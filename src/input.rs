use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum InputButton {
    Keyboard(KeyCode),
    Mouse(MouseButton),
}

impl InputButton {
    pub fn is_pressed(self) -> bool {
        match self {
            Self::Keyboard(key_code) => is_key_pressed(key_code),
            Self::Mouse(mouse_button) => is_mouse_button_pressed(mouse_button),
        }
    }

    pub fn is_down(self) -> bool {
        match self {
            Self::Keyboard(key_code) => is_key_down(key_code),
            Self::Mouse(mouse_button) => is_mouse_button_down(mouse_button),
        }
    }

    pub fn is_released(self) -> bool {
        match self {
            Self::Keyboard(key_code) => is_key_released(key_code),
            Self::Mouse(mouse_button) => is_mouse_button_released(mouse_button),
        }
    }
}

impl From<KeyCode> for InputButton {
    fn from(value: KeyCode) -> Self {
        Self::Keyboard(value)
    }
}

impl From<MouseButton> for InputButton {
    fn from(value: MouseButton) -> Self {
        Self::Mouse(value)
    }
}
