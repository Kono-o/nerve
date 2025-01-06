use crate::engine::{ButtonState, KeyBitMap, MouseBitMap};
use crate::renderer::core::{GLRenderer, VKRenderer};
use crate::renderer::{CamProj, NECamera, Renderer};
use crate::{
   NEError, NEEvents, NEGame, NEGameInfo, NERenderer, NEResult, NEWindow, ScreenCoord,
   ScreenOffset, Size2D,
};
use glfw::{
   Error, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, SwapInterval, WindowEvent, WindowHint,
};
use std::time::Instant;

#[derive(Copy, Clone)]
pub enum RenderAPI {
   OpenGL(u32, u32),
   Vulkan,
}

pub enum WinMode {
   Windowed(u32, u32),
   Full,
}
pub enum FPS {
   Vsync,
   Max,
}

pub struct NEGameBuilder {
   pub mode: WinMode,
   pub render_api: RenderAPI,
   pub title: String,
   pub fps: FPS,
}

impl Default for NEGameBuilder {
   fn default() -> Self {
      Self {
         render_api: RenderAPI::OpenGL(4, 5),
         title: "<Nerve-Game>".to_string(),
         mode: WinMode::Windowed(1280, 720),
         fps: FPS::Vsync,
      }
   }
}

fn window_from(
   glfw: &mut Glfw,
   mode: &WinMode,
   title: &str,
) -> NEResult<(PWindow, GlfwReceiver<(f64, WindowEvent)>, bool, Size2D)> {
   let mut is_fullscreen = false;
   let mut size = Size2D { w: 0, h: 0 };

   let window_and_events: NEResult<(PWindow, GlfwReceiver<(f64, WindowEvent)>)> = glfw
      .with_primary_monitor(|glfw, monitor| match monitor {
         None => {
            return NEResult::ER(NEError::Init {
               kind: NEInitErrKind::NoMonitor,
            })
         }

         Some(monitor) => {
            let vid_mode = match monitor.get_video_mode() {
               None => {
                  return NEResult::ER(NEError::Init {
                     kind: NEInitErrKind::NotVidMode,
                  })
               }
               Some(vm) => vm,
            };
            let mode = match mode {
               WinMode::Windowed(mut w, mut h) => {
                  const DIV: u32 = 3;
                  let min_h = vid_mode.height / DIV;
                  let min_w = vid_mode.width / DIV;

                  if w < min_w {
                     w = min_w;
                  };
                  if h < min_h {
                     h = min_h;
                  }

                  size.w = w;
                  size.h = h;
                  glfw::WindowMode::Windowed
               }
               WinMode::Full => {
                  is_fullscreen = true;
                  size.w = vid_mode.width;
                  size.h = vid_mode.height;
                  glfw::WindowMode::FullScreen(monitor)
               }
            };
            return match glfw.create_window(size.w, size.h, &title, mode) {
               None => NEResult::ER(NEError::Init {
                  kind: NEInitErrKind::CouldNotMakeWindow,
               }),
               Some(we) => NEResult::OK(we),
            };
         }
      });
   let (mut window, events) = match window_and_events {
      NEResult::OK((w, e)) => (w, e),
      NEResult::ER(e) => return NEResult::ER(e),
   };
   window.set_all_polling(true);
   window.set_framebuffer_size_polling(true);
   NEResult::OK((window, events, is_fullscreen, size))
}

fn init_nerve(
   glfw: &mut Glfw,
   api: &RenderAPI,
   mode: &WinMode,
   title: &str,
) -> NEResult<(
   Box<dyn Renderer>,
   PWindow,
   GlfwReceiver<(f64, WindowEvent)>,
   bool,
   Size2D,
)> {
   match api {
      RenderAPI::OpenGL(v0, v1) => {
         glfw.window_hint(WindowHint::ContextVersion(*v0, *v1));
         glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

         let (mut window, events, is_full, size) = match window_from(glfw, mode, title) {
            NEResult::OK((w, e, isf, s)) => (w, e, isf, s),
            NEResult::ER(e) => return NEResult::ER(e),
         };

         NEResult::OK((Box::new(GLRenderer), window, events, is_full, size))
      }
      RenderAPI::Vulkan => {
         glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

         let (window, events, is_full, size) = match window_from(glfw, mode, title) {
            NEResult::OK((w, e, isf, s)) => (w, e, isf, s),
            NEResult::ER(e) => return NEResult::ER(e),
         };

         NEResult::OK((Box::new(VKRenderer), window, events, is_full, size))
      }
   }
}

pub(crate) enum NEInitErrKind {
   GlfwInit,
   APIUnavailable(String),
   APIWrongVersion(String),
   NoMonitor,
   NotVidMode,
   WindowHasNoContext,
   CouldNotMakeWindow,
   Unknown(String),
}

impl NEGameBuilder {
   pub fn build(&self) -> NEResult<NEGame> {
      let api = self.render_api.clone();
      let glfw_error_log = move |err: Error, desc: String| {
         let api_str = match api {
            RenderAPI::OpenGL(v0, v1) => {
               format!("OpenGL {v0}.{v1}")
            }
            RenderAPI::Vulkan => "Vulkan".to_string(),
         };
         let kind = match err {
            Error::ApiUnavailable => NEInitErrKind::APIUnavailable(api_str),
            Error::VersionUnavailable | Error::InvalidValue => {
               NEInitErrKind::APIWrongVersion(api_str)
            }
            Error::NoWindowContext => NEInitErrKind::WindowHasNoContext,
            _ => NEInitErrKind::Unknown(desc),
         };
         NEError::Init { kind }.log_and_exit();
      };

      let mut glfw = match glfw::init(glfw_error_log) {
         Ok(g) => g,
         Err(_) => {
            return NEResult::ER(NEError::Init {
               kind: NEInitErrKind::GlfwInit,
            });
         }
      };
      let (core, mut window, events, is_fullscreen, size) =
         match init_nerve(&mut glfw, &self.render_api, &self.mode, &self.title) {
            NEResult::OK((c, w, e, isf, s)) => (c, w, e, isf, s),
            NEResult::ER(e) => return NEResult::ER(e),
         };
      core.init(&mut window);
      let (swap_interval, is_vsync) = match self.fps {
         FPS::Vsync => (SwapInterval::Adaptive, true),
         FPS::Max => (SwapInterval::None, false),
      };
      glfw.set_swap_interval(swap_interval);

      let (cx, cy) = (size.w / 2, size.h / 2);
      window.set_cursor_pos(cx as f64, cy as f64);
      let cam = NECamera::from(size, CamProj::Persp);
      let (x, y) = window.get_pos();
      let coord = ScreenCoord::from(x as f64, y as f64);
      let cursor_coord = ScreenCoord::from(cx as f64, cy as f64);
      let cursor_coord_global = ScreenCoord::from(cx as f64 + coord.x, cy as f64 + coord.y);

      let current_time = Instant::now();
      NEResult::OK(NEGame {
         renderer: NERenderer::from(core, self.render_api, cam.view_matrix, cam.proj_matrix),
         window: NEWindow {
            glfw: glfw.clone(),
            window,
            prev_cursor_coord: cursor_coord,
            cursor_offset: ScreenOffset::empty(),
            prev_coord: coord,
            prev_size: size,
            is_cursor_hidden: false,
            is_cursor_off: false,
            is_fullscreen,
            is_resizable: true,
            is_running: true,
            is_vsync,
            size,
            coord,
            title: self.title.clone(),
            cursor_coord,
            cursor_coord_global,
         },
         events: NEEvents {
            events,
            key_bitmap: KeyBitMap(
               [ButtonState {
                  pressed: false,
                  held: false,
                  released: false,
               }; 121],
            ),
            mouse_bitmap: MouseBitMap(
               [ButtonState {
                  pressed: false,
                  held: false,
                  released: false,
               }; 8],
            ),
            keys_to_reset: Vec::new(),
            mouse_to_reset: Vec::new(),
            window_resize_event: (false, size),
            window_close_event: false,
         },
         info: NEGameInfo {
            frame: 0,
            fps: 0.0,
            current_time,
            delta: 0.0,
            glfw,
            prev_time: current_time,
            prev_sec: current_time,
            local_frame: 0,
            time: 0.0,
            start_time: current_time,
            prev_deltas: vec![],
            prev_deltas_size: 128,
         },
         cam,
      })
   }
}
