use crate::NEGameRef;

pub trait NEObject {
   //on scene load/game startup
   fn start(&mut self, game: &mut NEGameRef);
   //start of a logic tick
   fn pre_update(&mut self, game: &mut NEGameRef);
   //same logic tick but after pre update
   fn update(&mut self, game: &mut NEGameRef);
   //end of a logic tick
   fn post_update(&mut self, game: &mut NEGameRef);

   //on scene unload/game exit

   fn end(&mut self, game: &mut NEGameRef);

   //render frame
   fn render(&mut self, game: &mut NEGameRef);
}
