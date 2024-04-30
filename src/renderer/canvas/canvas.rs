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
   pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl NerveCanvas {
   fn from(glfw: Glfw, window: PWindow, events: GlfwReceiver<(f64, WindowEvent)>) -> Self {
      Self {
         glfw,
         window,
         events,
      }
   }

   pub fn alive(&self) -> bool {
      !self.window.should_close()
   }

   pub fn time(&self) -> f64 {
      self.glfw.get_time()
   }

   pub fn post(&mut self) {
      self.window.swap_buffers();
      self.glfw.poll_events()
   }
}
