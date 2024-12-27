use crate::core::{GLRenderer, VKRenderer};
use crate::engine::{ButtonState, KeyBitMap, MouseBitMap};
use crate::renderer::{CamProj, NerveCamera, NerveRenderer, Renderer};
use crate::{
   NerveEvents, NerveGame, NerveGameInfo, NerveWindow, ScreenCoord, ScreenOffset, WinSize,
};
use glfw::{Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, SwapInterval, WindowEvent, WindowHint};

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

pub struct NerveGameBuilder {
   pub mode: WinMode,
   pub render_api: RenderAPI,
   pub title: String,
   pub fps: FPS,
}

impl Default for NerveGameBuilder {
   fn default() -> Self {
      Self {
         render_api: RenderAPI::OpenGL(3, 3),
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
) -> (PWindow, GlfwReceiver<(f64, WindowEvent)>, bool, WinSize) {
   let mut is_fullscreen = false;
   let mut size = WinSize { w: 0, h: 0 };

   let (mut window, events) = glfw.with_primary_monitor(|glfw, monitor| match monitor {
      None => panic!("no monitor found"),

      Some(mut monitor) => {
         let vid_mode = monitor.get_video_mode().expect("no video mode found");
         let mode = match mode {
            WinMode::Windowed(mut w, mut h) => {
               let min_size = vid_mode.height / 3;
               if w < min_size {
                  w = min_size;
               };
               if h < min_size {
                  h = min_size;
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
            None => panic!("failed to make window!"),
            Some(we) => return we,
         };
      }
   });
   window.set_all_polling(true);
   window.set_framebuffer_size_polling(true);
   (window, events, is_fullscreen, size)
}

fn init_nerve(
   glfw: &mut Glfw,
   api: &RenderAPI,
   mode: &WinMode,
   title: &str,
) -> (
   Box<dyn Renderer>,
   PWindow,
   GlfwReceiver<(f64, WindowEvent)>,
   bool,
   WinSize,
) {
   match api {
      RenderAPI::OpenGL(v0, v1) => {
         glfw.window_hint(WindowHint::ContextVersion(*v0, *v1));
         glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
         let (mut window, events, is_full, size) = window_from(glfw, mode, title);
         (Box::new(GLRenderer), window, events, is_full, size)
      }
      RenderAPI::Vulkan => {
         glfw.window_hint(WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
         let (mut window, events, is_full, size) = window_from(glfw, mode, title);
         (Box::new(VKRenderer), window, events, is_full, size)
      }
   }
}
fn glfw_error_log(_err: glfw::Error, desc: String) {
   println!("{}", desc.to_lowercase());
}

impl NerveGameBuilder {
   pub fn build(&self) -> NerveGame {
      let mut glfw = glfw::init(glfw_error_log).unwrap();
      let (core, mut window, events, is_fullscreen, size) =
         init_nerve(&mut glfw, &self.render_api, &self.mode, &self.title);
      core.init(&mut window, &mut glfw);
      let (swap_interval, is_vsync) = match self.fps {
         FPS::Vsync => (SwapInterval::Adaptive, true),
         FPS::Max => (SwapInterval::None, false),
      };
      glfw.set_swap_interval(swap_interval);

      let (cx, cy) = (size.w / 2, size.h / 2);
      window.set_cursor_pos(cx as f64, cy as f64);
      let cam = NerveCamera::from(size, CamProj::Persp);
      let coord = ScreenCoord::from_tup(window.get_pos());
      let cursor_coord = ScreenCoord::from(cx as i32, cy as i32);
      let cursor_coord_global = ScreenCoord::from(cx as i32 + coord.x, cy as i32 + coord.y);

      NerveGame {
         renderer: NerveRenderer::from(core, self.render_api, cam.view_matrix, cam.proj_matrix),
         window: NerveWindow {
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
         events: NerveEvents {
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
         info: NerveGameInfo {
            glfw,
            prev_time: 0.0,
            prev_sec: 0.0,
            local_frame: 0,
            frame: 0,
            fps: 0,
            time: 0.0,
            delta: 0.0,
         },
         cam,
      }
   }
}
