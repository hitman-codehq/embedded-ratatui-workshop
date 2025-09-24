use core::fmt;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonPressType {
    Short,
    Long,
}

impl fmt::Display for ButtonPressType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ButtonPressType::Short => write!(f, "Short Press"),
            ButtonPressType::Long => write!(f, "Long Press"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    Button1(ButtonPressType),
    Button2(ButtonPressType),
    Both,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Button::Button1(press_type) => write!(f, "Button 1 ({press_type})"),
            Button::Button2(press_type) => write!(f, "Button 2 ({press_type})"),
            Button::Both => write!(f, "Both Buttons"),
        }
    }
}

impl Button {
    pub fn is_mode(&self) -> bool {
        matches!(self, Button::Button1(_))
    }

    pub fn is_menu(&self) -> bool {
        matches!(self, Button::Button2(_))
    }

    pub fn is_short_press(&self) -> bool {
        matches!(
            self,
            Button::Button1(ButtonPressType::Short) | Button::Button2(ButtonPressType::Short)
        )
    }

    pub fn is_long_press(&self) -> bool {
        matches!(
            self,
            Button::Button1(ButtonPressType::Long) | Button::Button2(ButtonPressType::Long)
        )
    }
}

pub struct ButtonState {
    pressed_at: Option<Instant>,
}

impl ButtonState {
    pub fn new() -> Self {
        Self { pressed_at: None }
    }
}

impl ButtonState {
    pub fn update<F>(&mut self, is_pressed: bool, on_press: F)
    where
        F: FnOnce(ButtonPressType),
    {
        if is_pressed {
            // Button is currently down
            if self.pressed_at.is_none() {
                self.pressed_at = Some(Instant::now());
            }
        } else if let Some(pressed_at) = self.pressed_at.take() {
            // Button just released
            let duration = pressed_at.elapsed().as_millis() as u64;
            let press_type = if (500..2000).contains(&duration) {
                Some(ButtonPressType::Long)
            } else if duration < 500 {
                Some(ButtonPressType::Short)
            } else {
                None
            };

            if let Some(press_type) = press_type {
                on_press(press_type);
            }
        }
    }
}
