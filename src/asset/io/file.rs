use crate::util::{NEError, NEResult};
use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

pub(crate) enum NEFileErrKind {
   NoFile,
   NoPerms,
   NotValidPath,
   Unsupported,
   Unknown,
}

pub(crate) fn load_from_disk(path: &str) -> NEResult<fs::File> {
   let pathbuf = PathBuf::from(path);
   if !pathbuf.exists() {
      return NEResult::ER(NEError::File {
         path: path.to_string(),
         kind: NEFileErrKind::NoFile,
      });
   }
   match pathbuf.extension() {
      None => {
         return NEResult::ER(NEError::File {
            path: path.to_string(),
            kind: NEFileErrKind::NotValidPath,
         })
      }
      _ => {}
   }
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
