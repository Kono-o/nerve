use crate::api::Renderer;
use crate::game::{key_to_bitmap, mouse_to_bitmap};
use crate::renderer::{NerveCamera, NerveRenderer};
use crate::{Is, Mouse, NerveEvents, NerveGameInfo, NerveWindow};
use glfw::{flush_messages, Action, Context, Key, SwapInterval, WindowEvent, WindowMode};

#[derive(Copy, Clone)]
pub struct WinSize {
   pub w: u32,
   pub h: u32,
}
impl WinSize {
   pub fn from(w: u32, h: u32) -> Self {
      Self { w, h }
   }
}

pub struct NerveGame {
   pub renderer: NerveRenderer,
   pub window: NerveWindow,
   pub events: NerveEvents,
   pub info: NerveGameInfo,
   pub cam: NerveCamera,
}

//PRIVATE
impl NerveGame {
   fn catch_events(&mut self) {
      for (_f, event) in flush_messages(&self.events.events) {
         match event {
            WindowEvent::Key(k, _, a, _) => {
               let key_state_in_bitmap = &mut self.events.key_bit_map.0[key_to_bitmap(&k)];
               if let Action::Press = a {
                  key_state_in_bitmap.pressed = true;
                  key_state_in_bitmap.held = true
               } else if let Action::Release = a {
                  key_state_in_bitmap.held = false;
                  key_state_in_bitmap.released = true
               }
               self.events.keys_to_be_reset.push(k);
            }
            WindowEvent::MouseButton(m, a, _) => {
               let m = Mouse::from(m);
               let mouse_state_in_bitmap = &mut self.events.mouse_bit_map.0[mouse_to_bitmap(&m)];
               if let Action::Press = a {
                  mouse_state_in_bitmap.pressed = true;
                  mouse_state_in_bitmap.held = true
               } else if let Action::Release = a {
                  mouse_state_in_bitmap.held = false;
                  mouse_state_in_bitmap.released = true
               }
               self.events.mouse_to_be_reset.push(m);
            }
            WindowEvent::FramebufferSize(w, h) => {
               let new_size = WinSize {
                  w: w as u32,
                  h: h as u32,
               };
               self.window.size = new_size;
               self.renderer.resize(new_size);
               self.cam.resize(new_size);
            }
            _ => {}
         };
      }
   }
   fn reset_buttons(&mut self) {
      for key in &self.events.keys_to_be_reset {
         let key_state_in_bitmap = &mut self.events.key_bit_map.0[key_to_bitmap(key)];
         key_state_in_bitmap.pressed = false;
         key_state_in_bitmap.released = false;
      }
      for mouse in &self.events.mouse_to_be_reset {
         let mouse_state_in_bitmap = &mut self.events.mouse_bit_map.0[mouse_to_bitmap(mouse)];
         mouse_state_in_bitmap.pressed = false;
         mouse_state_in_bitmap.released = false;
      }
   }
   fn time_calc(&mut self) {
      self.info.time = self.window.glfw.get_time();

      let current = self.info.time;
      self.info.frame += 1;
      self.info.delta = (current - self.info.prev_time) as f32;
      self.info.prev_time = current;
      if current - self.info.prev_sec >= 1.0 {
         self.info.fps = self.info.frame as u32;
         self.info.frame = 0;
         self.info.prev_sec = current;
      }
   }
   fn mouse_offset_calc(&mut self) {
      let (x, y) = self.mouse_pos();
      self.info.mouse_pos_offset = (
         x as i32 - self.info.prev_mouse_pos.0 as i32,
         self.info.prev_mouse_pos.1 as i32 - y as i32,
      );
      self.info.prev_mouse_pos = (x, y);
   }
   fn pre_update(&mut self) {
      self.time_calc();
      self.catch_events();
      self.mouse_offset_calc();
      self.cam.recalc_view();
      self.renderer.draw_bg();
   }
   fn post_update(&mut self) {
      self.window.swap_buffers();
      self.reset_buttons();
      self.window.glfw.poll_events();
   }
}
//PUBLIC
impl NerveGame {
   pub fn pre(&mut self) {
      self.pre_update()
   }
   pub fn post(&mut self) {
      self.post_update()
   }

   pub fn alive(&self) -> bool {
      !self.window.should_close()
   }
   pub fn kill(&mut self) {
      self.window.set_should_close(true)
   }
   pub fn size(&mut self) -> WinSize {
      self.window.get_size()
   }

   pub fn resize(&mut self, size: WinSize) {
      self.window.set_size(size)
   }
   pub fn set_cam(&mut self, camera: NerveCamera) {
      self.cam = camera
   }
   pub fn enable_vsync(&mut self, enabled: bool) {
      self.window.glfw.set_swap_interval(match enabled {
         true => SwapInterval::Adaptive,
         false => SwapInterval::None,
      })
   }
   pub fn toggle_fullscreen(&mut self) {
      if self.window.is_fullscreen {
         self.window.set_monitor(
            WindowMode::Windowed,
            self.info.prev_pos,
            self.info.prev_size,
            None,
         );
      } else {
         self.info.prev_pos = self.window.get_pos();
         self.info.prev_size = self.window.get_size();

         self.window.glfw.with_primary_monitor(|_, m| {
            let monitor = m.expect("no monitor found!");
            let mode = monitor.get_video_mode().unwrap();
            self.window.window.set_monitor(
               WindowMode::FullScreen(&monitor),
               0,
               0,
               mode.width,
               mode.height,
               Some(mode.refresh_rate),
            );
         })
      }
      self.window.is_fullscreen = !self.window.is_fullscreen;
   }

   pub fn key(&self, key: Key, action: Is) -> bool {
      let key_state_in_bitmap = &self.events.key_bit_map.0[key_to_bitmap(&key)];
      match action {
         Is::Pressed => key_state_in_bitmap.pressed,
         Is::Released => key_state_in_bitmap.released,
         Is::Held => key_state_in_bitmap.held,
      }
   }
   pub fn mouse(&self, mouse: Mouse, action: Is) -> bool {
      let mouse_state_in_bitmap = &self.events.mouse_bit_map.0[mouse_to_bitmap(&mouse)];
      return match action {
         Is::Pressed => mouse_state_in_bitmap.pressed,
         Is::Released => mouse_state_in_bitmap.released,
         Is::Held => mouse_state_in_bitmap.held,
      };
   }
   pub fn mouse_pos(&self) -> (u32, u32) {
      let (x, y) = self.window.get_cursor_pos();
      return (x as u32, y as u32);
   }
   pub fn mouse_pos_offset(&self) -> (i32, i32) {
      return self.info.mouse_pos_offset;
   }
}
