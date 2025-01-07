use crate::renderer::NECamera;
use crate::{ansi, proc};
use crate::{log_info, NEEvents, NEGameInfo, NERenderer, NEWindow, Size2D};

pub struct NEGame {
   pub renderer: NERenderer,
   pub window: NEWindow,
   pub events: NEEvents,
   pub info: NEGameInfo,
   pub cam: NECamera,
   pub is_paused: bool,
}

impl NEGame {
   fn resize_children(&mut self, new_size: Size2D) {
      self.renderer.set_size(new_size);
      self.cam.set_size(new_size);
   }
   fn handle_events(&mut self) {
      if self.events.window_resize_event.0 {
         self.resize_children(self.events.window_resize_event.1);
         self.events.window_resize_event.0 = false;
      }

      if self.events.window_close_event {
         self.window.close();
      }
   }

   pub fn start(&mut self) {
      if self.window.is_hidden {
         self.window.set_visibility(true)
      }
      log_info!("{} started!", self.window.title);
   }
   pub fn pre_update(&mut self) {
      self.renderer.pre_update(&self.cam);
      self.window.pre_update();
      self.events.pre_update();
      self.info.pre_update();
      self.cam.pre_update();
      self.handle_events();
   }
   pub fn update(&mut self) {}
   pub fn post_update(&mut self) {
      self.renderer.post_update();
      self.window.post_update();
      self.events.post_update();
      self.info.post_update();
      self.cam.post_update();
   }
   pub fn end(self) {
      log_info!("{} ended!", self.window.title);
      drop(self);
   }
   pub fn end_and_exit(self) {
      self.end();
      proc::end_success()
   }

   pub fn set_cam(&mut self, camera: NECamera) {
      self.cam = camera
   }
   pub fn pause(&mut self) {
      self.is_paused = true;
   }
   pub fn unpause(&mut self) {
      self.is_paused = false;
   }
   pub fn toggle_pause(&mut self) {
      match self.is_paused {
         true => self.unpause(),
         false => self.pause(),
      }
   }
}
