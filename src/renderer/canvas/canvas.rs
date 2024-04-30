use glfw::*;

pub enum FPS {
   Vsync,
   Max,
}
pub struct NerveCanvasBuilder {
   pub ogl_version: (u32, u32),
   pub title: String,
   pub width: u32,
   pub height: u32,
   pub fps: FPS,
}

impl Default for NerveCanvasBuilder {
   fn default() -> Self {
      Self {
         ogl_version: (3, 3),
         title: "<Nerve-Canvas>".to_string(),
         width: 1280,
         height: 720,
         fps: FPS::Vsync,
      }
   }
}

impl NerveCanvasBuilder {
   pub fn build(&self) -> NerveCanvas {
      match init(glfw::fail_on_errors) {
         Err(error) => panic!("glfw: {}", error),
         Ok(mut glfw) => {
            glfw.window_hint(WindowHint::ContextVersion(
               self.ogl_version.0,
               self.ogl_version.1,
            ));
            glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
            match glfw.create_window(self.width, self.height, &self.title, WindowMode::Windowed) {
               None => panic!("canvas: failed to build window."),
               Some((mut window, events)) => {
                  window.make_current();
                  window.set_key_polling(true);
                  window.set_framebuffer_size_polling(true);
                  gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
                  glfw.set_swap_interval(match self.fps {
                     FPS::Vsync => SwapInterval::Adaptive,
                     FPS::Max => SwapInterval::None,
                  });
                  NerveCanvas::from(glfw, window, events)
               }
            }
         }
      }
   }
}

pub struct NerveCanvas {
   glfw: Glfw,
   window: PWindow,
   events: GlfwReceiver<(f64, WindowEvent)>,
   pub frame: u64,
   prev_time: f64,
   prev_sec: f64,
   pub delta: f64,
}

impl NerveCanvas {
   fn from(glfw: Glfw, window: PWindow, events: GlfwReceiver<(f64, WindowEvent)>) -> Self {
      Self {
         glfw,
         window,
         events,
         frame: 0,
         prev_time: 0.0,
         prev_sec: 0.0,
         delta: 0.0,
      }
   }
   pub fn alive(&self) -> bool {
      !self.window.should_close()
   }
   pub fn kill(&mut self) {
      self.window.set_should_close(true)
   }

   pub fn mouse_pos(&self) -> (u32, u32) {
      let (x, y) = self.window.get_cursor_pos();
      return (x as u32, y as u32);
   }

   pub fn key_events(&self) -> Vec<(Key, Is)> {
      let mut events = Vec::new();
      for (_, event) in flush_messages(&self.events) {
         match event {
            WindowEvent::Key(k, _, a, _) => events.push((k, Is::from(a))),
            _ => {}
         };
      }
      events
   }

   pub fn set_size(&mut self, width: u32, height: u32) {
      self.window.set_size(width as i32, height as i32)
   }
   pub fn time(&self) -> f64 {
      self.glfw.get_time()
   }

   fn fps(&mut self) {
      let current = self.time();
      self.frame += 1;
      self.delta = current - self.prev_time;
      self.prev_time = current;
      if current - self.prev_sec >= 1.0 {
         println!("fps: {}", self.frame);
         self.frame = 0;
         self.prev_sec = current;
      }
   }

   pub fn pre(&mut self) {
      self.fps();
   }

   pub fn post(&mut self) {
      self.window.swap_buffers();
      self.glfw.poll_events()
   }
}

pub enum Is {
   Pressed,
   Released,
   Held,
}

impl Is {
   fn from(act: Action) -> Self {
      match act {
         Action::Release => Is::Released,
         Action::Press => Is::Pressed,
         Action::Repeat => Is::Held,
      }
   }
}
