use glfw::*;
use crate::{Fps, Is, Mouse, NerveEvents, NerveRenderer};

pub struct NerveCanvas {
   glfw: Glfw,
   window: PWindow,
   events: GlfwReceiver<(f64, WindowEvent)>,
   is_fullscreen: bool,
   prev_pos: (i32, i32),
   prev_size: (i32, i32),
   prev_time: f64,
   prev_sec: f64,
   frame: u64,

   pub fps: u32,
   pub time: f64,
   pub delta: f64,
}

impl NerveCanvas {
   pub(crate) fn make(
      glfw: Glfw,
      window: PWindow,
      events: GlfwReceiver<(f64, WindowEvent)>,
      is_fullscreen: bool,
   ) -> Self {
      Self {
         glfw,
         window,
         events,
         is_fullscreen,
         prev_time: 0.0,
         prev_sec: 0.0,
         prev_pos: (0, 0),
         prev_size: (0, 0),
         frame: 0,

         fps: 0,
         time: 0.0,
         delta: 0.0,
      }
   }

   pub fn mouse_pos(&self) -> (u32, u32) {
      let (x, y) = self.window.get_cursor_pos();
      return (x as u32, y as u32);
   }

   pub fn events(&mut self) -> NerveEvents {
      let mut key = Vec::new();
      let mut mouse = Vec::new();
      for (_f, event) in flush_messages(&self.events) {
         match event {
            WindowEvent::Key(k, _, a, _) => key.push((k, Is::from(a))),
            WindowEvent::MouseButton(m, a, _) => mouse.push((Mouse::from(m), Is::from(a))),
            WindowEvent::FramebufferSize(width, height) if false => unsafe {
               gl::Viewport(0, 0, width, height)
            },
            _ => {}
         };
      }
      NerveEvents { key, mouse }
   }

   fn time_calc(&mut self) {
      self.time = self.glfw.get_time();

      let current = self.time;
      self.frame += 1;
      self.delta = current - self.prev_time;
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
      NerveRenderer::fill();
   }

   pub fn post(&mut self) {
      self.window.swap_buffers();
      self.glfw.poll_events();
   }

   pub fn alive(&self) -> bool {
      !self.window.should_close()
   }

   pub fn kill(&mut self) {
      self.window.set_should_close(true)
   }

   pub fn set_size(&mut self, width: u32, height: u32) {
      self.window.set_size(width as i32, height as i32)
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
   pub fn set_vsync(&mut self, vsync: Fps) {
      self.glfw.set_swap_interval(match vsync {
         Fps::Vsync => SwapInterval::Adaptive,
         Fps::Max => SwapInterval::None,
      })
   }
}
