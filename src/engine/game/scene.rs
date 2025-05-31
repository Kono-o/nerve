use crate::{ansi, log_event, log_warn, CamProj, NEGameRef, Size2D};
use crate::{NECamera, NERuntime};

pub struct NESceneRef<'a> {
   pub cam: &'a mut NECamera,
   pub world: &'a mut hecs::World,
}

pub struct NEScene {
   pub(crate) name: String,
   pub(crate) world: hecs::World,
   pub(crate) runtime: Box<dyn NERuntime>,
   pub(crate) cam: NECamera,
}

struct DefaultRuntime; //PLACEHOLDER

impl NERuntime for DefaultRuntime {
   fn start(&mut self, game: NEGameRef, scene: NESceneRef) {
      log_warn!(
         "default runtime is running!\nuse scene.replace_runtime() to add custom behaviour!"
      );
   }
   fn pre_update(&mut self, _game: NEGameRef, _scene: NESceneRef) {}
   fn update(&mut self, _game: NEGameRef, _scene: NESceneRef) {}
   fn post_update(&mut self, _game: NEGameRef, _scene: NESceneRef) {}
   fn end(&mut self, _game: NEGameRef, _scene: NESceneRef) {}
}

impl NEScene {
   pub fn new(name: &str) -> NEScene {
      NEScene {
         name: name.to_string(),
         world: hecs::World::new(),
         runtime: Box::from(DefaultRuntime),
         cam: NECamera::new(Size2D::from(1, 1), CamProj::Persp),
      }
   }

   pub fn set_runtime(&mut self, runtime: Box<dyn NERuntime>) {
      self.runtime = runtime
   }
   pub fn replace_cam(&mut self, cam: NECamera) {
      self.cam = cam;
   }
}

impl NEScene {
   pub(crate) fn start(&mut self, game: NEGameRef) {
      log_event!("scene [{}] begun!", self.name);
      self.cam.start();
      let scene = NESceneRef {
         cam: &mut self.cam,
         world: &mut self.world,
      };
      self.runtime.start(game, scene);
   }

   pub(crate) fn pre_update(&mut self, game: NEGameRef) {
      self.cam.pre_update();
      let scene = NESceneRef {
         cam: &mut self.cam,
         world: &mut self.world,
      };
      self.runtime.pre_update(game, scene);
   }

   pub(crate) fn update(&mut self, game: NEGameRef) {
      self.cam.update();
      let scene = NESceneRef {
         cam: &mut self.cam,
         world: &mut self.world,
      };
      self.runtime.update(game, scene);
   }

   pub(crate) fn post_update(&mut self, game: NEGameRef) {
      self.cam.post_update();
      let scene = NESceneRef {
         cam: &mut self.cam,
         world: &mut self.world,
      };
      self.runtime.post_update(game, scene);
   }

   pub(crate) fn end(&mut self, game: NEGameRef) {
      self.cam.end();
      let scene = NESceneRef {
         cam: &mut self.cam,
         world: &mut self.world,
      };
      self.runtime.end(game, scene);
      log_event!("scene [{}] ended!", self.name);
   }
}
