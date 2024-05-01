use glfw::{Action, Key, MouseButton};

pub struct NerveEvents {
   pub key: Vec<(Key, Is)>,
   pub mouse: Vec<(Mouse, Is)>,
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
