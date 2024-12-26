use crate::api::{GLRenderer, Renderer, VKRenderer};
use crate::game::{ButtonState, KeyBitMap, MouseBitMap};
use crate::renderer::{CamProj, NerveCamera, NerveRenderer};
use crate::{NerveEvents, NerveGame, NerveGameInfo, NerveWindow, WinSize};
use glfw::{Glfw, GlfwReceiver, OpenGlProfileHint, PWindow, WindowEvent, WindowHint};

pub enum RenderAPI {
   OpenGL(u32, u32),
   Vulkan,
}

pub enum WinMode {
   Windowed(u32, u32),
   Full,
}
pub enum Frame {
   Vsync,
   Max,
}

pub struct NerveGameBuilder {
   pub renderer: RenderAPI,
   pub title: String,
   pub mode: WinMode,
   pub fps: Frame,
}

impl Default for NerveGameBuilder {
   fn default() -> Self {
      Self {
         renderer: RenderAPI::OpenGL(3, 3),
         title: "<Nerve-Game>".to_string(),
         mode: WinMode::Windowed(1280, 720),
         fps: Frame::Max,
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

fn create_from(
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

impl NerveGameBuilder {
   pub fn build(&self) -> NerveGame {
      let mut glfw = glfw::init(glfw::fail_on_errors).expect("glfw init failed");
      let (context, mut window, events, is_fullscreen, size) =
         create_from(&mut glfw, &self.renderer, &self.mode, &self.title);
      context.init(&mut window, &mut glfw);
      //glfw.set_swap_interval(match self.fps {
      //   Frame::Vsync => SwapInterval::Adaptive,
      //   Frame::Max => SwapInterval::None,
      //});

      NerveGame {
         renderer: NerveRenderer::new(context),
         window: NerveWindow {
            glfw,
            window,
            is_fullscreen,
            size,
         },
         events: NerveEvents {
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
         },
         info: NerveGameInfo {
            prev_mouse_pos: (0, 0),
            mouse_pos_offset: (0, 0),
            prev_pos: (0, 0),
            prev_size: size,
            prev_time: 0.0,
            prev_sec: 0.0,
            frame: 0,
            fps: 0,
            time: 0.0,
            delta: 0.0,
         },
         cam: NerveCamera::new(size, CamProj::Perspective),
      }
   }
}
