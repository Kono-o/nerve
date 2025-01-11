use crate::engine::game::cycle::NECycle;
use crate::{ansi, log_event, proc, NEScene};
use crate::{NEEvents, NERenderer, NETime, NEWindow, Size2D};

pub struct NEGame {
   pub renderer: NERenderer,
   pub window: NEWindow,
   pub events: NEEvents,
   pub cycle: NECycle,
   pub scene: NEScene,
   pub time: NETime,
}

impl NEGame {
   fn resize_children(&mut self, new_size: Size2D) {
      self.renderer.set_size(new_size);
      self.scene.cam.set_size(new_size);
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

   pub fn replace_scene(&mut self, scene: NEScene) {
      self.scene = scene;
      self.resize_children(self.window.size)
   }

   pub fn start(&mut self) {
      log_event!("game [{}] run!", self.window.title);
      if self.window.is_hidden {
         self.window.set_visibility(true)
      }
      self.scene.start(
         &mut self.renderer,
         &mut self.window,
         &mut self.events,
         &mut self.cycle,
         &mut self.time,
      );
   }

   fn should_poll(&self) -> bool {
      if self.time.frame % 30 == 0 {
         return true;
      }
      false
   }
   pub fn pre_update(&mut self) {
      self.renderer.pre_update(&self.scene.cam);
      self.window.pre_update();
      self.events.pre_update(self.should_poll());
      self.time.pre_update();
      self.handle_events();
      self.scene.pre_update(
         &mut self.renderer,
         &mut self.window,
         &mut self.events,
         &mut self.cycle,
         &mut self.time,
      );
   }

   pub fn update(&mut self) {
      if !self.cycle.is_paused {
         self.scene.update(
            &mut self.renderer,
            &mut self.window,
            &mut self.events,
            &mut self.cycle,
            &mut self.time,
         )
      }
   }

   pub fn post_update(&mut self) {
      self.scene.post_update(
         &mut self.renderer,
         &mut self.window,
         &mut self.events,
         &mut self.cycle,
         &mut self.time,
      );
      self.renderer.post_update();
      self.window.post_update();
      self.events.post_update();
      self.time.post_update();
   }
   pub fn end(mut self) {
      self.scene.end(
         &mut self.renderer,
         &mut self.window,
         &mut self.events,
         &mut self.cycle,
         &mut self.time,
      );
      log_event!("game [{}] end!", self.window.title);
      drop(self);
   }
   pub fn end_and_exit(self) {
      self.end();
      proc::end_success()
   }

   pub fn render(&mut self) {
      self.scene.render(
         &mut self.renderer,
         &mut self.window,
         &mut self.events,
         &mut self.cycle,
         &mut self.time,
      );
   }
}
