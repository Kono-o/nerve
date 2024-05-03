use cgmath::{Deg, perspective, Vector3};
use glfw::*;
use crate::{CamProj, NerveCamera, NerveCanvas};

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
   pub opengl_version: (u32, u32),
   pub title: String,
   pub mode: CanvasMode,
   pub fps: Fps,
}

impl Default for NerveCanvasBuilder {
   fn default() -> Self {
      Self {
         opengl_version: (3, 3),
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
      let mut glfw = glfw_init(self.opengl_version);
      let mut is_fullscreen = false;
      let camera;
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

      let (width, height) = window.get_size();
      let (widthf, heightf) = (width as f32, height as f32);

      let proj_matrix = perspective(Deg(50.0), widthf / heightf, 0.01, 1000.0);

      camera = NerveCamera {
         size: (width as u32, width as u32),
         projection: CamProj::Perspective,
         fov: 50.0,
         clip: (0.01, 1000.0),
         proj_matrix,
         position: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
         },
         rotation: Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
         },
      };
      NerveCanvas::make(glfw, window, events, is_fullscreen, camera)
   }
}
