use crate::Size2D;
use glfw::{flush_messages, Action, Glfw, GlfwReceiver, Key, MouseButton, WindowEvent};
use std::time::Instant;

#[derive(Copy, Clone)]
pub(crate) struct ButtonState {
   pub(crate) pressed: bool,
   pub(crate) held: bool,
   pub(crate) released: bool,
}

pub(crate) struct KeyBitMap(pub(crate) [ButtonState; 121]);
pub(crate) struct MouseBitMap(pub(crate) [ButtonState; 8]);

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

pub(crate) fn mouse_index(mouse: &Mouse) -> usize {
   match mouse {
      Mouse::Left => 0,
      Mouse::Right => 1,
      Mouse::Middle => 2,
      Mouse::Button4 => 3,
      Mouse::Button5 => 4,
      Mouse::Button6 => 5,
      Mouse::Button7 => 6,
      Mouse::Button8 => 7,
   }
}
pub(crate) fn key_index(key: &Key) -> usize {
   match key {
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
   }
}

pub struct NerveEvents {
   pub(crate) events: GlfwReceiver<(f64, WindowEvent)>,
   pub(crate) key_bitmap: KeyBitMap,
   pub(crate) mouse_bitmap: MouseBitMap,
   pub(crate) keys_to_reset: Vec<Key>,
   pub(crate) mouse_to_reset: Vec<Mouse>,
   pub(crate) window_resize_event: (bool, Size2D),
   pub(crate) window_close_event: bool,
}

impl NerveEvents {
   pub(crate) fn pre_update(&mut self) {
      self.catch()
   }
   pub(crate) fn post_update(&mut self) {
      self.reset()
   }

   fn catch(&mut self) {
      for (_f, event) in flush_messages(&self.events) {
         match event {
            WindowEvent::Key(k, _, a, _) => {
               let key_in_bitmap = &mut self.key_bitmap.0[key_index(&k)];
               if let Action::Press = a {
                  key_in_bitmap.pressed = true;
                  key_in_bitmap.held = true
               } else if let Action::Release = a {
                  key_in_bitmap.held = false;
                  key_in_bitmap.released = true
               }
               self.keys_to_reset.push(k);
            }
            WindowEvent::MouseButton(m, a, _) => {
               let m = Mouse::from(m);
               let mouse_in_bitmap = &mut self.mouse_bitmap.0[mouse_index(&m)];
               if let Action::Press = a {
                  mouse_in_bitmap.pressed = true;
                  mouse_in_bitmap.held = true
               } else if let Action::Release = a {
                  mouse_in_bitmap.held = false;
                  mouse_in_bitmap.released = true
               }
               self.mouse_to_reset.push(m);
            }
            WindowEvent::FramebufferSize(w, h) => {
               let new_size = Size2D {
                  w: w as u32,
                  h: h as u32,
               };
               self.window_resize_event = (true, new_size);
            }
            WindowEvent::Close => {
               self.window_close_event = true;
            }
            _ => {}
         }
      }
   }
   fn reset(&mut self) {
      for key in &self.keys_to_reset {
         let key_in_bitmap = &mut self.key_bitmap.0[key_index(key)];
         key_in_bitmap.pressed = false;
         key_in_bitmap.released = false;
      }
      for mouse in &self.mouse_to_reset {
         let mouse_in_bitmap = &mut self.mouse_bitmap.0[mouse_index(mouse)];
         mouse_in_bitmap.pressed = false;
         mouse_in_bitmap.released = false;
      }
   }

   pub fn key(&self, key: Key, action: Is) -> bool {
      let key_in_bitmap = &self.key_bitmap.0[key_index(&key)];
      match action {
         Is::Pressed => key_in_bitmap.pressed,
         Is::Released => key_in_bitmap.released,
         Is::Held => key_in_bitmap.held,
      }
   }
   pub fn mouse(&self, mouse: Mouse, action: Is) -> bool {
      let mouse_in_bitmap = &self.mouse_bitmap.0[mouse_index(&mouse)];
      match action {
         Is::Pressed => mouse_in_bitmap.pressed,
         Is::Released => mouse_in_bitmap.released,
         Is::Held => mouse_in_bitmap.held,
      }
   }
}

pub struct NerveGameInfo {
   pub fps: f64,
   pub time: f64,
   pub delta: f64,
   pub frame: u64,

   pub(crate) glfw: Glfw,
   pub(crate) prev_sec: Instant,
   pub(crate) prev_time: Instant,
   pub(crate) prev_deltas: Vec<f64>,
   pub(crate) prev_deltas_size: usize,
   pub(crate) start_time: Instant,
   pub(crate) current_time: Instant,
   pub(crate) local_frame: u32,
}

impl NerveGameInfo {
   pub(crate) fn pre_update(&mut self) {
      self.calculate()
   }
   pub(crate) fn post_update(&mut self) {}

   fn calculate(&mut self) {
      self.frame += 1;
      self.local_frame += 1;

      self.current_time = Instant::now();
      self.time = self
         .current_time
         .duration_since(self.start_time)
         .as_secs_f64();
      self.delta = self
         .current_time
         .duration_since(self.prev_time)
         .as_secs_f64();
      self.prev_time = self.current_time;

      self.prev_deltas.push(self.delta);
      if self.prev_deltas.len() > self.prev_deltas_size {
         self.prev_deltas.remove(0);
      }

      let avg_delta = self.prev_deltas.iter().sum::<f64>() / self.prev_deltas.len() as f64;
      self.fps = 1.0 / avg_delta;

      if self
         .current_time
         .duration_since(self.prev_sec)
         .as_secs_f32()
         >= 1.0
      {
         self.local_frame = 0;
         self.prev_sec = self.current_time
      }
   }
}

pub enum Is {
   Pressed,
   Released,
   Held,
}
