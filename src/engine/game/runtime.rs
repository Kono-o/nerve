use crate::{NEGameRef, NESceneRef};

pub trait NERuntime {
   //on scene load/game startup
   fn start(&mut self, game: NEGameRef, scene: NESceneRef);
   //start of a logic tick
   fn pre_update(&mut self, game: NEGameRef, scene: NESceneRef);
   //same logic tick but after pre update
   fn update(&mut self, game: NEGameRef, scene: NESceneRef);
   //end of a logic tick
   fn post_update(&mut self, game: NEGameRef, scene: NESceneRef);
   //on scene unload/game exit
   fn end(&mut self, game: NEGameRef, scene: NESceneRef);
}
