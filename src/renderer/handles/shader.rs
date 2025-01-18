use crate::NETexture;
use cgmath::Matrix4;

pub enum TexSlot {
   S0,
   S1,
   S2,
   S3,
   S4,
   S5,
   S6,
   S7,
   S8,
   S9,
   S10,
}

impl TexSlot {
   pub(crate) fn as_index(&self) -> usize {
      match self {
         TexSlot::S0 => 0,
         TexSlot::S1 => 1,
         TexSlot::S2 => 2,
         TexSlot::S3 => 3,
         TexSlot::S4 => 4,
         TexSlot::S5 => 5,
         TexSlot::S6 => 6,
         TexSlot::S7 => 7,
         TexSlot::S8 => 8,
         TexSlot::S9 => 9,
         TexSlot::S10 => 10,
      }
   }
}

#[derive(Clone, Debug)]
pub struct NEShader {
   pub(crate) id: u32,
   pub(crate) tex_ids: Vec<Option<u32>>,
}

impl NEShader {
   pub(crate) fn temporary() -> NEShader {
      let mut tex_ids = Vec::new();
      tex_ids.resize(12, None);
      NEShader { id: 0, tex_ids }
   }
   pub fn attach_tex(&mut self, tex: &NETexture) {
      for (slot, tex_id) in self.tex_ids.iter().enumerate() {
         match tex_id {
            None => {
               println!("(attach) at s {} id {}", slot, tex.id);
               self.tex_ids[slot] = Some(tex.id);
               break;
            }
            Some(_) => {}
         }
      }
   }

   pub fn set_tex_at_slot(&mut self, tex: &NETexture, slot: TexSlot) {
      self.tex_ids[slot.as_index()] = Some(tex.id)
   }
}

pub enum Uniform {
   Matrix4(Matrix4<f32>),
   Int(i32),
}
