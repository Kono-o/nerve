use crate::util::{NEError, NEResult};
use std::fs;
use std::io::ErrorKind;

pub(crate) enum NEFileErrKind {
   NoFile,
   NoPerms,
   NotValidPath,
   Unsupported,
   Unknown,
}

pub(crate) fn load_from_disk(path: &str) -> NEResult<fs::File> {
   let mut errkind;
   match fs::File::open(path) {
      Ok(f) => NEResult::OK(f),
      Err(e) => {
         errkind = match e.kind() {
            ErrorKind::NotFound => NEFileErrKind::NoFile,
            ErrorKind::PermissionDenied => NEFileErrKind::NoPerms,
            ErrorKind::InvalidInput => NEFileErrKind::NotValidPath,
            _ => NEFileErrKind::Unknown,
         };
         NEResult::ER(NEError::File {
            path: path.to_string(),
            kind: errkind,
         })
      }
   }
}
