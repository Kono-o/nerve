use glfw::*;
use crate::NerveCanvas;

pub struct CanvasSize {
   pub width: u32,
   pub height: u32,
}

pub enum CanvasMode {
   Windowed(CanvasSize),
   FullScreen,
}

pub enum Fps {
   Vsync,
   Max,
}

pub struct NerveCanvasBuilder {
   pub ogl_version: (u32, u32),
   pub title: String,
   pub mode: CanvasMode,
   pub fps: Fps,
}

impl Default for NerveCanvasBuilder {
   fn default() -> Self {
      Self {
         ogl_version: (3, 3),
         title: "<Nerve-Canvas>".to_string(),
         mode: CanvasMode::Windowed(CanvasSize {
            width: 960,
            height: 540,
         }),
         fps: Fps::Vsync,
      }
   }
}

fn glfw_init(v: (u32, u32)) -> Glfw {
   match init(glfw::fail_on_errors) {
      Err(error) => panic!("glfw: {}", error),
      Ok(mut glfw) => {
         glfw.window_hint(WindowHint::ContextVersion(v.0, v.1));
         glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
         return glfw;
      }
   }
}

fn window_init(window: &mut PWindow) {
   window.make_current();
   //window.set_key_polling(true);
   window.set_framebuffer_size_polling(true);
   //window.set_mouse_button_polling(true);
   window.set_all_polling(true);
   gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);
}

impl NerveCanvasBuilder {
   pub fn build(&self) -> NerveCanvas {
      let mut glfw = glfw_init(self.ogl_version);
      let mut is_fullscreen = false;

      let (mut window, events) = glfw.with_primary_monitor(|glfw, monitor| match monitor {
         None => panic!("no monitor found"),
         Some(mut monitor) => {
            let (mode, width, height) = match self.mode {
               CanvasMode::Windowed(CanvasSize {
                  width: w,
                  height: h,
               }) => (WindowMode::Windowed, w, h),
               CanvasMode::FullScreen => {
                  is_fullscreen = true;
                  let vid_mode = monitor.get_video_mode().unwrap();
                  (
                     WindowMode::FullScreen(&mut monitor),
                     vid_mode.width,
                     vid_mode.height,
                  )
               }
            };
            match glfw.create_window(width, height, &self.title, mode) {
               None => panic!("failed to make canvas!"),
               Some(we) => return we,
            };
         }
      });
      window_init(&mut window);
      glfw.set_swap_interval(match self.fps {
         Fps::Vsync => SwapInterval::Adaptive,
         Fps::Max => SwapInterval::None,
      });
      NerveCanvas::make(glfw, window, events, is_fullscreen)
   }
}
