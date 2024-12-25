use crate::NerveWindow;
use glfw::*;

pub enum WindowMode {
   Windowed(u32, u32),
   FullScreen,
}
pub enum Fps {
   Vsync,
   Max,
}

pub struct NerveWindowBuilder {
   pub opengl_version: (u32, u32),
   pub title: String,
   pub mode: WindowMode,
   pub fps: Fps,
}

fn glfw_init(v: (u32, u32)) -> Glfw {
   match init(glfw::fail_on_errors) {
      Err(error) => panic!("glfw: {}", error),
      Ok(mut glfw) => {
         glfw.window_hint(WindowHint::ContextVersion(v.0, v.1));
         glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
         glfw
      }
   }
}
fn window_init(window: &mut PWindow) {
   window.make_current();
   window.set_all_polling(true);
   window.set_framebuffer_size_polling(true);
   gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
}

impl Default for NerveWindowBuilder {
   fn default() -> Self {
      Self {
         opengl_version: (3, 3),
         title: "<Nerve-Window>".to_string(),
         mode: WindowMode::Windowed(1280, 720),
         fps: Fps::Vsync,
      }
   }
}
impl NerveWindowBuilder {
   pub fn build(&self) -> NerveWindow {
      let mut glfw = glfw_init(self.opengl_version);
      let mut is_fullscreen = false;
      let (mut window, events) = glfw.with_primary_monitor(|glfw, monitor| match monitor {
         None => panic!("no monitor found"),
         Some(mut monitor) => {
            let vid_mode = monitor.get_video_mode().unwrap();
            let (mode, width, height) = match self.mode {
               WindowMode::Windowed(mut w, mut h) => {
                  let min_size = vid_mode.height / 2;
                  if w < min_size {
                     w = min_size;
                  };
                  if h < min_size {
                     h = min_size;
                  }
                  (glfw::WindowMode::Windowed, w, h)
               }
               WindowMode::FullScreen => {
                  is_fullscreen = true;
                  (
                     glfw::WindowMode::FullScreen(monitor),
                     vid_mode.width,
                     vid_mode.height,
                  )
               }
            };
            match glfw.create_window(width, height, &self.title, mode) {
               None => panic!("failed to make window!"),
               Some(we) => return we,
            };
         }
      });
      window_init(&mut window);
      glfw.set_swap_interval(match self.fps {
         Fps::Vsync => SwapInterval::Adaptive,
         Fps::Max => SwapInterval::None,
      });
      NerveWindow::make(glfw, window, events, is_fullscreen)
   }
}
