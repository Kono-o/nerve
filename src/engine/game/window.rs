use crate::{ScreenCoord, ScreenOffset, WinSize};
use glfw::{Context, CursorMode, Glfw, PWindow, SwapInterval, WindowMode};

pub struct NerveWindow {
   pub(crate) glfw: Glfw,
   pub(crate) window: PWindow,

   pub(crate) prev_cursor_coord: ScreenCoord,
   pub(crate) prev_coord: ScreenCoord,
   pub(crate) prev_size: WinSize,

   pub is_cursor_hidden: bool,
   pub is_cursor_off: bool,
   pub is_fullscreen: bool,
   pub is_resizable: bool,
   pub is_running: bool,
   pub is_vsync: bool,

   pub size: WinSize,
   pub coord: ScreenCoord,
   pub title: String,
   pub cursor_coord: ScreenCoord,
   pub cursor_offset: ScreenOffset,
}

impl NerveWindow {
   pub(crate) fn set_monitor(
      &mut self,
      mode: WindowMode,
      prev_pos: ScreenCoord,
      prev_size: WinSize,
      refresh_rate: Option<u32>,
   ) {
      self.window.set_monitor(
         mode,
         prev_pos.x,
         prev_pos.y,
         prev_size.w,
         prev_size.h,
         refresh_rate,
      );
   }
   pub(crate) fn pre_update(&mut self) {
      self.glfw.poll_events();
      self.size = self.get_size();
      self.coord = self.get_coord();
      self.cursor_coord = self.get_cursor_coord();
      self.cursor_offset = self.get_cursor_offset();
   }
   pub(crate) fn post_update(&mut self) {
      self.swap();
   }

   fn get_size(&mut self) -> WinSize {
      let (w, h) = self.window.get_size();
      WinSize::from(w as u32, h as u32)
   }
   fn get_coord(&self) -> ScreenCoord {
      ScreenCoord::from_tup(self.window.get_pos())
   }
   fn get_cursor_coord(&self) -> ScreenCoord {
      let (x, y) = self.window.get_cursor_pos();
      ScreenCoord::from(x as i32, y as i32)
   }
   fn get_cursor_offset(&mut self) -> ScreenOffset {
      let coord = self.cursor_coord;
      let cursor_offset = ScreenOffset::from(
         coord.x - self.prev_cursor_coord.x,
         self.prev_cursor_coord.y - coord.y,
      );
      self.prev_cursor_coord = coord;
      cursor_offset
   }

   fn swap(&mut self) {
      self.window.swap_buffers()
   }

   pub fn close(&mut self) {
      self.window.set_should_close(true);
      self.is_running = false
   }

   pub fn select(&mut self) {
      self.window.make_current()
   }

   pub fn set_title(&mut self, title: String) {
      self.window.set_title(&title);
      self.title = title;
   }
   pub fn set_size(&mut self, size: WinSize) {
      self.window.set_size(size.w as i32, size.h as i32);
   }
   pub fn set_coord(&mut self, coord: ScreenCoord) {
      self.coord = coord;
      self.window.set_pos(coord.x, coord.y);
   }

   pub fn cursor_is_inside(&self) -> bool {
      self.cursor_coord.is_inside(self.size)
   }
   pub fn set_cursor_pos(&mut self, coord: ScreenCoord) {
      self.cursor_coord = coord;
      self.window.set_cursor_pos(coord.x as f64, coord.y as f64)
   }

   pub fn set_cursor_visibility(&mut self, hide: bool) {
      self.is_cursor_hidden = hide;
      if !self.is_cursor_off {
         self.window.set_cursor_mode(match hide {
            true => CursorMode::Normal,
            false => CursorMode::Hidden,
         });
      }
   }
   pub fn toggle_cursor_visibility(&mut self) {
      self.is_cursor_hidden = !self.is_cursor_hidden;
      self.set_cursor_visibility(self.is_cursor_hidden);
   }

   pub fn set_cursor_usage(&mut self, enable: bool) {
      if self.is_cursor_off != enable {
         self.is_cursor_off = enable;
         self.toggle_cursor_usage()
      }
   }
   pub fn toggle_cursor_usage(&mut self) {
      self.is_cursor_off = !self.is_cursor_off;
      self.window.set_cursor_mode(match self.is_cursor_off {
         true => CursorMode::Disabled,
         false => match self.is_cursor_hidden {
            true => CursorMode::Hidden,
            false => CursorMode::Normal,
         },
      })
   }

   pub fn set_fullscreen(&mut self, enable: bool) {
      if self.is_fullscreen != enable {
         self.is_fullscreen = enable;
         self.toggle_fullscreen()
      }
   }
   pub fn toggle_fullscreen(&mut self) {
      self.is_fullscreen = !self.is_fullscreen;
      if self.is_fullscreen {
         self.prev_coord = self.get_coord();
         self.prev_size = self.get_size();

         self.glfw.with_primary_monitor(|_, m| {
            let monitor = m.expect("no monitor found!");
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
      } else {
         self.set_monitor(
            WindowMode::Windowed,
            self.prev_coord,
            self.prev_size.shave(1),
            None,
         );
      }
   }
   pub fn set_resizable(&mut self, enable: bool) {
      if self.is_resizable != enable {
         self.is_resizable = enable;
         self.toggle_resizable()
      }
   }
   pub fn toggle_resizable(&mut self) {
      self.is_resizable = !self.is_resizable;
      self.window.set_resizable(self.is_resizable);
   }
   pub fn set_vsync(&mut self, enable: bool) {
      self.window.glfw.set_swap_interval(match enable {
         true => SwapInterval::Adaptive,
         false => SwapInterval::None,
      });
      self.is_vsync = enable;
   }
   pub fn toggle_vsync(&mut self) {
      self.is_vsync = !self.is_vsync;
      self.set_vsync(self.is_vsync);
   }
}
