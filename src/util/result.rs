use crate::util::NEError;

pub enum NEResult<N> {
   OK(N),
   ER(NEError),
}

impl<N> NEResult<N> {
   pub fn unwrap(self) -> N {
      match self {
         NEResult::OK(n) => n,
         NEResult::ER(e) => {
            e.log();
            panic!()
         }
      }
   }

   pub fn is_ok(&self) -> bool {
      match self {
         NEResult::OK(_) => true,
         NEResult::ER(_) => false,
      }
   }
   pub fn is_err(&self) -> bool {
      !self.is_ok()
   }
}
