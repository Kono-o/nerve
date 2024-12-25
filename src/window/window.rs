use crate::window::events::{key_to_bitmap, mouse_to_bitmap, ButtonState, KeyBitMap, MouseBitMap};
use crate::{Is, Mouse, NerveCamera, NerveRenderer};
use glfw::*;

pub struct NerveWindow {
   glfw: Glfw,
   window: PWindow,
   events: GlfwReceiver<(f64, WindowEvent)>,

   key_bit_map: KeyBitMap,
   mouse_bit_map: MouseBitMap,
   keys_to_be_reset: Vec<Key>,
   mouse_to_be_reset: Vec<Mouse>,

   is_fullscreen: bool,
   size: (i32, i32),
   prev_mouse_pos: (u32, u32),
   mouse_pos_offset: (i32, i32),

   prev_pos: (i32, i32),
   prev_size: (i32, i32),
   prev_time: f64,
   prev_sec: f64,
   frame: u64,

   pub cam: NerveCamera,
   pub renderer: NerveRenderer,
   pub fps: u32,
   pub time: f64,
   pub delta: f32,
}

impl NerveWindow {
   pub(crate) fn make(
      glfw: Glfw,
      window: PWindow,
      events: GlfwReceiver<(f64, WindowEvent)>,
      is_fullscreen: bool,
   ) -> Self {
      let size = window.get_size();
      let mut renderer = NerveRenderer::default();
      renderer.init();

      Self {
         glfw,
         window,
         events,

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
         size,
         prev_mouse_pos: ((size.0 / 2) as u32, (size.1 / 2) as u32),
         mouse_pos_offset: (0, 0),
         prev_time: 0.0,
         prev_sec: 0.0,
         prev_pos: (200, 200),
         prev_size: (720, 720),
         frame: 0,

         cam: NerveCamera::default(size.0, size.1),
         renderer,
         fps: 0,
         time: 0.0,
         delta: 0.0,
      }
   }
   fn catch_events(&mut self) {
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
               self.size = (w, h);
               self.renderer.resize(w, h);
               self.cam.resize(w, h);
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
         self.frame = 0;
         self.prev_sec = current;
      }
   }
   fn mouse_offset_calc(&mut self) {
      let (x, y) = self.mouse_pos();
      self.mouse_pos_offset = (
         x - self.prev_mouse_pos.0 as i32,
         self.prev_mouse_pos.1 as i32 - y as i32,
      );
      self.prev_mouse_pos.0 = x as u32;
      self.prev_mouse_pos.1 = y as u32;
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
      self.glfw.poll_events();
   }
}

impl NerveWindow {
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
   pub fn size(&self) -> (i32, i32) {
      self.window.get_size()
   }

   pub fn resize(&mut self, width: u32, height: u32) {
      self.window.set_size(width as i32, height as i32)
   }
   pub fn set_cam(&mut self, camera: NerveCamera) {
      self.cam = camera
   }
   pub fn set_renderer(&mut self, renderer: NerveRenderer) {
      self.renderer = renderer
   }
   pub fn enable_vsync(&mut self, enabled: bool) {
      self.glfw.set_swap_interval(match enabled {
         true => SwapInterval::Adaptive,
         false => SwapInterval::None,
      })
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
         );
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
   pub fn mouse_pos(&self) -> (i32, i32) {
      let (x, y) = self.window.get_cursor_pos();
      return (x as i32, y as i32);
   }
   pub fn mouse_pos_offset(&self) -> (i32, i32) {
      return self.mouse_pos_offset;
   }
}
