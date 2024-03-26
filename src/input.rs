use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct InputAxis {
    pub positive: (Vec<InputButton>, AxisState),
    pub negative: (Vec<InputButton>, AxisState),
}

impl InputAxis {
    pub fn from_inputs(positive: Vec<InputButton>, negative: Vec<InputButton>) -> Self {
        let positive = (positive, AxisState::Off);
        let negative = (negative, AxisState::Off);
        Self { positive, negative }
    }

    pub fn update_state(&mut self) {
        Self::update(
            (&self.positive.0).into_iter().any(|b| b.is_down()),
            &mut self.positive.1,
            &mut self.negative.1,
        );

        Self::update(
            (&self.negative.0).into_iter().any(|b| b.is_down()),
            &mut self.negative.1,
            &mut self.positive.1,
        );
    }

    fn update(input: bool, to_update: &mut AxisState, other: &mut AxisState) {
        match (input, *to_update, *other) {
            (true, AxisState::Pressed, AxisState::Off) => {
                *to_update = AxisState::Active;
            }
            (true, AxisState::Off, AxisState::Active) => {
                *to_update = AxisState::Active;
                *other = AxisState::Pressed;
            }
            (true, AxisState::Off, _) => {
                *to_update = AxisState::Active;
            }
            (false, AxisState::Off, _) => {}
            (false, _, _) => {
                *to_update = AxisState::Off;
            }
            _ => {}
        }
    }

    pub fn as_i8(&self) -> i8 {
        match (self.positive.1, self.negative.1) {
            (AxisState::Active, _) => 1,
            (_, AxisState::Active) => -1,
            _ => 0,
        }
    }

    pub fn as_f32(&self) -> f32 {
        self.as_i8() as f32
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AxisState {
    Off,
    Active,
    Pressed,
}

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
