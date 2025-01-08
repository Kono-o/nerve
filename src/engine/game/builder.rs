use crate::engine::{ButtonState, KeyBitMap, MouseBitMap};
use crate::renderer::core::{GLRenderer, VKRenderer};
use crate::renderer::{CamProj, NECamera, Renderer};
use crate::util::{NEError, NEResult};
use crate::{
   NEEvents, NEGame, NERenderer, NEScene, NETime, NEWindow, ScreenCoord, ScreenOffset, Size2D,
};
use glfw::{
   Error, Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, SwapInterval, WindowEvent, WindowHint,
};
use std::fmt::{Display, Formatter};
use std::time::Instant;

#[derive(Copy, Clone)]
pub enum RenderAPI {
   OpenGL(u32, u32),
   Vulkan(u32, u32),
}

impl Display for RenderAPI {
   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.api_str())
   }
}

impl RenderAPI {
   pub(crate) fn api_str(&self) -> String {
      match self {
         RenderAPI::OpenGL(v0, v1) => format!("OpenGL {v0}.{v1}"),
         RenderAPI::Vulkan(v0, v1) => format!("Vulkan {v0}.{v1}"),
      }
   }
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
   pub decorated: bool,
   pub title: String,
   pub fps: FPS,
}

impl Default for NEGameBuilder {
   fn default() -> Self {
      Self {
         render_api: RenderAPI::OpenGL(4, 5),
         decorated: true,
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
) -> NEResult<(
   PWindow,
   GlfwReceiver<(f64, WindowEvent)>,
   bool,
   Size2D,
   Size2D,
)> {
   let mut is_fullscreen = false;
   let mut size = Size2D { w: 0, h: 0 };
   let mut monitor_size = Size2D { w: 0, h: 0 };

   let window_and_events: NEResult<(PWindow, GlfwReceiver<(f64, WindowEvent)>)> = glfw
      .with_primary_monitor(|glfw, monitor| {
         return match monitor {
            None => NEResult::ER(NEError::Init {
               kind: NEInitErrKind::NoMonitor,
            }),

            Some(monitor) => {
               let vid_mode = match monitor.get_video_mode() {
                  None => {
                     return NEResult::ER(NEError::Init {
                        kind: NEInitErrKind::NoVidMode,
                     })
                  }
                  Some(vm) => vm,
               };
               monitor_size = Size2D::from(vid_mode.width, vid_mode.height);

               let mode = match mode {
                  WinMode::Windowed(mut w, mut h) => {
                     const DIV: u32 = 10;
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
               match glfw.create_window(size.w, size.h, &title, mode) {
                  None => NEResult::ER(NEError::Init {
                     kind: NEInitErrKind::CouldNotMakeWindow,
                  }),
                  Some(we) => NEResult::OK(we),
               }
            }
         };
      });
   let (mut window, events) = match window_and_events {
      NEResult::OK((w, e)) => (w, e),
      NEResult::ER(e) => return NEResult::ER(e),
   };
   window.hide();
   window.set_all_polling(true);
   window.set_framebuffer_size_polling(true);
   NEResult::OK((window, events, is_fullscreen, size, monitor_size))
}

fn init_nerve(
   glfw: &mut Glfw,
   api: &RenderAPI,
   decorated: bool,
   mode: &WinMode,
   title: &str,
) -> NEResult<(
   Box<dyn Renderer>,
   PWindow,
   GlfwReceiver<(f64, WindowEvent)>,
   bool,
   Size2D,
   Size2D,
)> {
   match api {
      RenderAPI::OpenGL(v0, v1) => {
         glfw.window_hint(WindowHint::ContextVersion(*v0, *v1));
         glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Compat));
         glfw.window_hint(WindowHint::Samples(Some(4)));
         glfw.window_hint(WindowHint::Decorated(decorated));

         let (mut window, events, is_full, size, monitor_size) =
            match window_from(glfw, mode, title) {
               NEResult::OK((w, e, isf, s, ms)) => (w, e, isf, s, ms),
               NEResult::ER(e) => return NEResult::ER(e),
            };

         NEResult::OK((
            Box::new(GLRenderer),
            window,
            events,
            is_full,
            size,
            monitor_size,
         ))
      }
      RenderAPI::Vulkan(_v0, _v1) => {
         glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

         let (window, events, is_full, size, monitor_size) = match window_from(glfw, mode, title) {
            NEResult::OK((w, e, isf, s, ms)) => (w, e, isf, s, ms),
            NEResult::ER(e) => return NEResult::ER(e),
         };

         NEResult::OK((
            Box::new(VKRenderer),
            window,
            events,
            is_full,
            size,
            monitor_size,
         ))
      }
   }
}

pub(crate) enum NEInitErrKind {
   GlfwInit,
   APIUnavailable(String),
   APIWrongVersion(String),
   APIUnsupported(String),
   NoMonitor,
   NoVidMode,
   WindowHasNoContext,
   CouldNotMakeWindow,
   Unknown(String),
}

impl NEGameBuilder {
   pub fn build(&self) -> NEResult<NEGame> {
      let api_str = self.render_api.api_str();
      let api_str_m = api_str.clone();
      match self.render_api {
         RenderAPI::Vulkan(_, _) => {
            return NEResult::ER(NEError::Init {
               kind: NEInitErrKind::APIUnsupported(api_str),
            })
         }
         _ => {}
      }
      let glfw_error_log = move |err: Error, desc: String| {
         let api = api_str_m.clone();
         let kind = match err {
            Error::ApiUnavailable => NEInitErrKind::APIUnavailable(api),
            Error::VersionUnavailable | Error::InvalidValue => NEInitErrKind::APIWrongVersion(api),
            Error::NoWindowContext => NEInitErrKind::WindowHasNoContext,
            _ => NEInitErrKind::Unknown(desc),
         };
         NEError::Init { kind }.log();
      };

      let mut glfw = match glfw::init(glfw_error_log) {
         Ok(g) => g,
         Err(_) => {
            return NEResult::ER(NEError::Init {
               kind: NEInitErrKind::GlfwInit,
            });
         }
      };
      let (core, mut window, events, is_fullscreen, window_size, monitor_size) = match init_nerve(
         &mut glfw,
         &self.render_api,
         self.decorated,
         &self.mode,
         &self.title,
      ) {
         NEResult::OK((c, w, e, isf, s, ms)) => (c, w, e, isf, s, ms),
         NEResult::ER(e) => return NEResult::ER(e),
      };
      core.init(api_str, &mut window);
      let (swap_interval, is_vsync) = match self.fps {
         FPS::Vsync => (SwapInterval::Adaptive, true),
         FPS::Max => (SwapInterval::None, false),
      };
      glfw.set_swap_interval(swap_interval);

      let (cx, cy) = (window_size.w / 2, window_size.h / 2);
      let (mx, my) = (monitor_size.w / 2, monitor_size.h / 2);
      let (wx, wy) = (mx - cx, my - cy); //centre of monitor
      let window_coord = ScreenCoord::from(wx as f64, wy as f64);

      let cursor_coord = ScreenCoord::from(cx as f64, cy as f64);
      let cursor_coord_global =
         ScreenCoord::from(cx as f64 + window_coord.x, cy as f64 + window_coord.y);

      let current_time = Instant::now();
      let mut window = NEWindow {
         glfw: glfw.clone(),
         window,
         prev_cursor_coord: cursor_coord,
         cursor_offset: ScreenOffset::empty(),
         prev_coord: window_coord,
         prev_size: window_size,
         is_cursor_hidden: false,
         is_cursor_off: false,
         is_fullscreen,
         is_hidden: true,
         is_borderless: !self.decorated,
         is_resizable: true,
         is_running: true,
         is_vsync,
         size: window_size,
         coord: window_coord,
         title: self.title.clone(),
         cursor_coord,
         cursor_coord_global,
      };
      window.set_coord(window_coord);

      let cam = NECamera::from(window_size, CamProj::Persp);
      let renderer = NERenderer::from(core, self.render_api, cam.view_matrix, cam.proj_matrix);
      let scene_name = format!("{}-init-scene", window.title);
      let mut scene = NEScene::new(&scene_name);
      scene.replace_cam(cam);

      NEResult::OK(NEGame {
         renderer,
         window,
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
            window_resize_event: (false, window_size),
            window_close_event: false,
         },
         time: NETime {
            frame: 0,
            fps: 0.0,
            current_time,
            delta: 0.0,
            glfw,
            prev_time: current_time,
            prev_sec: current_time,
            local_frame: 0,
            elapsed: 0.0,
            start_time: current_time,
            prev_deltas: vec![],
            prev_deltas_size: 128,
         },
         scene,
         is_paused: false,
      })
   }
}
