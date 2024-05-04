use glfw::*;
use crate::{Is, Mouse, NerveCamera, NerveRenderer};
use crate::renderer::canvas::events::{
   key_to_bitmap, KeyBitMap, ButtonState, MouseBitMap, mouse_to_bitmap,
};

pub struct NerveCanvas {
   glfw: Glfw,
   window: PWindow,
   events: GlfwReceiver<(f64, WindowEvent)>,

   key_bit_map: KeyBitMap,
   mouse_bit_map: MouseBitMap,
   keys_to_be_reset: Vec<Key>,
   mouse_to_be_reset: Vec<Mouse>,

   is_fullscreen: bool,
   prev_pos: (i32, i32),
   prev_size: (i32, i32),
   prev_time: f64,
   prev_sec: f64,
   frame: u64,

   pub cam: NerveCamera,
   pub fps: u32,
   pub time: f64,
   pub delta: f32,
}

impl NerveCanvas {
   pub(crate) fn make(
      glfw: Glfw,
      window: PWindow,
      events: GlfwReceiver<(f64, WindowEvent)>,
      is_fullscreen: bool,
      camera: NerveCamera,
   ) -> Self {
      Self {
         glfw,
         window,
         events,
         cam: camera,
         key_bit_map: KeyBitMap(
            [ButtonState {
               pressed: false,
               held: false,
               released: false,
            }; 121],
         ),
         mouse_bit_map: MouseBitMap(
            [ButtonState {
               pressed: false,
               held: false,
               released: false,
            }; 8],
         ),
         keys_to_be_reset: Vec::new(),
         mouse_to_be_reset: Vec::new(),
         is_fullscreen,
         prev_time: 0.0,
         prev_sec: 0.0,
         prev_pos: (200, 200),
         prev_size: (720, 720),
         frame: 0,

         fps: 0,
         time: 0.0,
         delta: 0.0,
      }
   }
   fn catch_buttons(&mut self) {
      for (_f, event) in flush_messages(&self.events) {
         match event {
            WindowEvent::Key(k, _, a, _) => {
               let key_state_in_bitmap = &mut self.key_bit_map.0[key_to_bitmap(&k)];
               if let Action::Press = a {
                  key_state_in_bitmap.pressed = true;
                  key_state_in_bitmap.held = true
               } else if let Action::Release = a {
                  key_state_in_bitmap.held = false;
                  key_state_in_bitmap.released = true
               }
               self.keys_to_be_reset.push(k);
            }
            WindowEvent::MouseButton(m, a, _) => {
               let m = Mouse::from(m);
               let mouse_state_in_bitmap = &mut self.mouse_bit_map.0[mouse_to_bitmap(&m)];
               if let Action::Press = a {
                  mouse_state_in_bitmap.pressed = true;
                  mouse_state_in_bitmap.held = true
               } else if let Action::Release = a {
                  mouse_state_in_bitmap.held = false;
                  mouse_state_in_bitmap.released = true
               }
               self.mouse_to_be_reset.push(m);
            }
            WindowEvent::FramebufferSize(w, h) => {
               NerveRenderer::resize(w, h);
               self.cam.resize(w as u32, h as u32);
               self.cam.recalc_proj()
            }
            _ => {}
         };
      }
   }
   fn reset_buttons(&mut self) {
      for key in &self.keys_to_be_reset {
         let key_state_in_bitmap = &mut self.key_bit_map.0[key_to_bitmap(key)];
         key_state_in_bitmap.pressed = false;
         key_state_in_bitmap.released = false;
      }
      for mouse in &self.mouse_to_be_reset {
         let mouse_state_in_bitmap = &mut self.mouse_bit_map.0[mouse_to_bitmap(mouse)];
         mouse_state_in_bitmap.pressed = false;
         mouse_state_in_bitmap.released = false;
      }
   }
   fn time_calc(&mut self) {
      self.time = self.glfw.get_time();

      let current = self.time;
      self.frame += 1;
      self.delta = (current - self.prev_time) as f32;
      self.prev_time = current;
      if current - self.prev_sec >= 1.0 {
         self.fps = self.frame as u32;
         println!("fps: {}", self.fps);
         self.frame = 0;
         self.prev_sec = current;
      }
   }
}

impl NerveCanvas {
   pub fn pre(&mut self) {
      self.time_calc();
      self.catch_buttons();
      NerveRenderer::fill();
      self.cam.recalc_view();
   }
   pub fn post(&mut self) {
      self.window.swap_buffers();
      self.glfw.poll_events();
      self.reset_buttons();
   }

   pub fn alive(&self) -> bool {
      !self.window.should_close()
   }
   pub fn kill(&mut self) {
      self.window.set_should_close(true)
   }
   pub fn size(&self) -> (i32, i32) {
      self.window.get_size()
   }
   pub fn set_size(&mut self, width: u32, height: u32) {
      self.window.set_size(width as i32, height as i32)
   }
   pub fn set_cam(&mut self, camera: NerveCamera) {
      self.cam = camera
   }
   pub fn key(&self, key: Key, action: Is) -> bool {
      let key_state_in_bitmap = &self.key_bit_map.0[key_to_bitmap(&key)];
      return match action {
         Is::Pressed => key_state_in_bitmap.pressed,
         Is::Released => key_state_in_bitmap.released,
         Is::Held => key_state_in_bitmap.held,
      };
   }
   pub fn mouse(&self, mouse: Mouse, action: Is) -> bool {
      let mouse_state_in_bitmap = &self.mouse_bit_map.0[mouse_to_bitmap(&mouse)];
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
   pub fn toggle_fullscreen(&mut self) {
      if self.is_fullscreen {
         self.window.set_monitor(
            WindowMode::Windowed,
            self.prev_pos.0,
            self.prev_pos.1,
            self.prev_size.0 as u32,
            self.prev_size.1 as u32,
            None,
         )
      } else {
         self.prev_pos = self.window.get_pos();
         self.prev_size = self.window.get_size();

         self.glfw.with_primary_monitor(|_, m| {
            let monitor = m.unwrap();

            let mode = monitor.get_video_mode().unwrap();

            self.window.set_monitor(
               WindowMode::FullScreen(&monitor),
               0,
               0,
               mode.width,
               mode.height,
               Some(mode.refresh_rate),
            );
         })
      }
      self.is_fullscreen = !self.is_fullscreen;
   }

   pub fn set_vsync(&mut self, enabled: bool) {
      self.glfw.set_swap_interval(match enabled {
         true => SwapInterval::Adaptive,
         false => SwapInterval::None,
      })
   }
}
