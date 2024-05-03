use glfw::{Action, Key, MouseButton};

#[derive(Copy, Clone)]
pub(crate) struct ButtonState {
   pub(crate) pressed: bool,
   pub(crate) held: bool,
   pub(crate) released: bool,
}

pub(crate) struct KeyBitMap(pub(crate) [ButtonState; 121]);
pub(crate) struct MouseBitMap(pub(crate) [ButtonState; 8]);

pub(crate) fn mouse_to_bitmap(mouse: &Mouse) -> usize {
   return match mouse {
      Mouse::Left => 0,
      Mouse::Right => 1,
      Mouse::Middle => 2,
      Mouse::Button4 => 3,
      Mouse::Button5 => 4,
      Mouse::Button6 => 5,
      Mouse::Button7 => 6,
      Mouse::Button8 => 7,
   };
}

pub(crate) fn key_to_bitmap(key: &Key) -> usize {
   return match key {
      Key::Space => 0,
      Key::Apostrophe => 1,
      Key::Comma => 2,
      Key::Minus => 3,
      Key::Period => 4,
      Key::Slash => 5,
      Key::Num0 => 6,
      Key::Num1 => 7,
      Key::Num2 => 8,
      Key::Num3 => 9,
      Key::Num4 => 10,
      Key::Num5 => 11,
      Key::Num6 => 12,
      Key::Num7 => 13,
      Key::Num8 => 14,
      Key::Num9 => 15,
      Key::Semicolon => 16,
      Key::Equal => 17,
      Key::A => 18,
      Key::B => 19,
      Key::C => 20,
      Key::D => 21,
      Key::E => 22,
      Key::F => 23,
      Key::G => 24,
      Key::H => 25,
      Key::I => 26,
      Key::J => 27,
      Key::K => 28,
      Key::L => 29,
      Key::M => 30,
      Key::N => 31,
      Key::O => 32,
      Key::P => 33,
      Key::Q => 34,
      Key::R => 35,
      Key::S => 36,
      Key::T => 37,
      Key::U => 38,
      Key::V => 39,
      Key::W => 40,
      Key::X => 41,
      Key::Y => 42,
      Key::Z => 43,
      Key::LeftBracket => 44,
      Key::Backslash => 45,
      Key::RightBracket => 46,
      Key::GraveAccent => 47,
      Key::World1 => 48,
      Key::World2 => 49,
      Key::Escape => 50,
      Key::Enter => 51,
      Key::Tab => 52,
      Key::Backspace => 53,
      Key::Insert => 54,
      Key::Delete => 55,
      Key::Right => 56,
      Key::Left => 57,
      Key::Down => 58,
      Key::Up => 59,
      Key::PageUp => 60,
      Key::PageDown => 61,
      Key::Home => 62,
      Key::End => 63,
      Key::CapsLock => 64,
      Key::ScrollLock => 65,
      Key::NumLock => 66,
      Key::PrintScreen => 67,
      Key::Pause => 68,
      Key::F1 => 69,
      Key::F2 => 70,
      Key::F3 => 71,
      Key::F4 => 72,
      Key::F5 => 73,
      Key::F6 => 74,
      Key::F7 => 75,
      Key::F8 => 76,
      Key::F9 => 77,
      Key::F10 => 78,
      Key::F11 => 79,
      Key::F12 => 80,
      Key::F13 => 81,
      Key::F14 => 82,
      Key::F15 => 83,
      Key::F16 => 84,
      Key::F17 => 85,
      Key::F18 => 86,
      Key::F19 => 87,
      Key::F20 => 88,
      Key::F21 => 89,
      Key::F22 => 90,
      Key::F23 => 91,
      Key::F24 => 92,
      Key::F25 => 93,
      Key::Kp0 => 94,
      Key::Kp1 => 95,
      Key::Kp2 => 96,
      Key::Kp3 => 97,
      Key::Kp4 => 98,
      Key::Kp5 => 99,
      Key::Kp6 => 100,
      Key::Kp7 => 101,
      Key::Kp8 => 102,
      Key::Kp9 => 103,
      Key::KpDecimal => 104,
      Key::KpDivide => 105,
      Key::KpMultiply => 106,
      Key::KpSubtract => 107,
      Key::KpAdd => 108,
      Key::KpEnter => 109,
      Key::KpEqual => 110,
      Key::LeftShift => 111,
      Key::LeftControl => 112,
      Key::LeftAlt => 113,
      Key::LeftSuper => 114,
      Key::RightShift => 115,
      Key::RightControl => 116,
      Key::RightAlt => 117,
      Key::RightSuper => 118,
      Key::Menu => 119,
      Key::Unknown => 120,
   };
}

#[derive(Debug)]
pub enum Mouse {
   Left,
   Right,
   Middle,
   Button4,
   Button5,
   Button6,
   Button7,
   Button8,
}

impl Mouse {
   pub(crate) fn from(mouse: MouseButton) -> Self {
      match mouse {
         MouseButton::Button1 => Self::Left,
         MouseButton::Button2 => Self::Right,
         MouseButton::Button3 => Self::Middle,
         MouseButton::Button4 => Self::Button4,
         MouseButton::Button5 => Self::Button5,
         MouseButton::Button6 => Self::Button6,
         MouseButton::Button7 => Self::Button7,
         MouseButton::Button8 => Self::Button8,
      }
   }
}
#[derive(Debug)]
pub enum Is {
   Pressed,
   Released,
   Held,
}

impl Is {
   pub(crate) fn from(act: Action) -> Self {
      match act {
         Action::Release => Self::Released,
         Action::Press => Self::Pressed,
         Action::Repeat => Self::Held,
      }
   }
}
