use crate::util::{NEError, NEResult};
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;

pub(crate) enum NEFileErrKind {
   NoFile,
   NoPath,
   NoPerms,
   NotValidPath,
   NotValidName,
   Unsupported,
   CouldNotCreate,
   CouldNotWrite,
   Unknown,
}

pub(crate) fn find_on_disk(path: &str) -> NEResult<fs::File> {
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

pub(crate) fn write_str_to_disk(path: &str, name: &str, content: &str) -> NEResult<()> {
   write_bytes_to_disk(path, name, content.as_bytes())
}

pub(crate) fn write_bytes_to_disk(path: &str, name: &str, content: &[u8]) -> NEResult<()> {
   let pathbuf = PathBuf::from(path);
   if !pathbuf.exists() {
      return NEResult::ER(NEError::File {
         path: path.to_string(),
         kind: NEFileErrKind::NoPath,
      });
   }
   let file_path = format!("{}{}", path, name);
   let mut file = match fs::File::create(&file_path) {
      Ok(f) => f,
      Err(_) => {
         return NEResult::ER(NEError::File {
            path: file_path,
            kind: NEFileErrKind::CouldNotCreate,
         });
      }
   };
   match file.write_all(content) {
      Ok(_) => NEResult::OK(()),
      Err(_) => NEResult::ER(NEError::File {
         path: file_path,
         kind: NEFileErrKind::CouldNotWrite,
      }),
   }
}

pub(crate) trait ReadBytes {
   fn read_as_bytes(&mut self, path: &str) -> NEResult<Vec<u8>>;
}

impl ReadBytes for fs::File {
   fn read_as_bytes(&mut self, path: &str) -> NEResult<Vec<u8>> {
      let mut buffer: Vec<u8> = Vec::new();
      let mut errkind;
      match self.read_to_end(&mut buffer) {
         Ok(_) => NEResult::OK(buffer),
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
}
