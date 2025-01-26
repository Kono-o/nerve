use crate::{ansi, log_event, proc, NERenderer, NEScene};
use crate::{NEEvents, NETime, NEWindow, Size2D};

pub struct NEGameRef<'a> {
   pub renderer: &'a mut NERenderer,
   pub window: &'a mut NEWindow,
   pub events: &'a mut NEEvents,
   pub time: &'a mut NETime,
}

pub struct NEGame {
   pub renderer: NERenderer,
   pub window: NEWindow,
   pub events: NEEvents,
   pub scene: NEScene,
   pub time: NETime,
}

impl NEGame {
   fn resize_children(&mut self, new_size: Size2D) {
      self.renderer.set_size(new_size);
      self.scene.cam.set_size(new_size);
   }
   fn handle_events(&mut self) {
      let mut resized = self.events.window_resize_event.0;
      if resized {
         self.resize_children(self.events.window_resize_event.1);
         resized = false;
      }

      if self.events.window_close_event {
         self.window.close();
      }
   }

   pub fn replace_scene(&mut self, scene: NEScene) {
      self.scene = scene;
      self.resize_children(self.window.size)
   }

   pub fn start(&mut self) {
      log_event!("game [{}] run!", self.window.title);
      let game = NEGameRef {
         renderer: &mut self.renderer,
         window: &mut self.window,
         events: &mut self.events,
         time: &mut self.time,
      };
      self.scene.start(game);
      if self.window.is_hidden {
         self.window.set_visibility(true)
      }
   }

   pub fn pre_update(&mut self) {
      self.time.pre_update();
      self.events.pre_update();
      self.window.pre_update();
      self.handle_events();
      self.renderer.pre_update(&self.scene.cam);
      let game = NEGameRef {
         renderer: &mut self.renderer,
         window: &mut self.window,
         events: &mut self.events,
         time: &mut self.time,
      };
      self.scene.pre_update(game);
   }

   pub fn update(&mut self) {
      let game = NEGameRef {
         renderer: &mut self.renderer,
         window: &mut self.window,
         events: &mut self.events,
         time: &mut self.time,
      };
      self.scene.update(game)
   }

   pub fn post_update(&mut self) {
      let game = NEGameRef {
         renderer: &mut self.renderer,
         window: &mut self.window,
         events: &mut self.events,
         time: &mut self.time,
      };
      self.scene.post_update(game);
      self.renderer.post_update();
      self.window.post_update();
      self.events.post_update();
      self.time.post_update();
   }
   pub fn end(mut self) {
      let game = NEGameRef {
         renderer: &mut self.renderer,
         window: &mut self.window,
         events: &mut self.events,
         time: &mut self.time,
      };
      self.scene.end(game);
      log_event!("game [{}] end!", self.window.title);
      drop(self);
   }

   pub fn end_and_exit(self) {
      self.end();
      proc::end_success()
   }
}
