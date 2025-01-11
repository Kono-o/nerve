use crate::util::{NEError, NEResult};
use crate::NEOption;
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;

pub(crate) enum NEFileErrKind {
   NoPerms,
   Missing,
   NotValid,
   Unsupported(String),
   CouldNotMake,
   CouldNotRead,
   CouldNotWrite,
   Unknown,
}

pub(crate) fn name(path: &str) -> NEOption<String> {
   let path = PathBuf::from(&path);
   match path.file_stem() {
      None => NEOption::Empty,
      Some(n) => NEOption::Exists(n.to_string_lossy().to_string()),
   }
}

pub(crate) fn ex(path: &str) -> NEOption<String> {
   let path = PathBuf::from(&path);
   match path.extension() {
      None => NEOption::Empty,
      Some(n) => NEOption::Exists(n.to_string_lossy().to_string()),
   }
}

pub(crate) fn exists_on_disk(path: &str) -> bool {
   let path = PathBuf::from(&path);
   path.exists()
}

pub(crate) fn write_str_to_disk(path: &str, name: &str, content: &str) -> NEResult<()> {
   write_bytes_to_disk(path, name, content.as_bytes())
}

pub(crate) fn write_bytes_to_disk(path: &str, name: &str, content: &[u8]) -> NEResult<()> {
   let pathbuf = PathBuf::from(path);
   if !pathbuf.exists() {
      //return NEResult::ER(NEError::file_missing(path));
      match fs::create_dir_all(path) {
         Err(_) => return NEResult::ER(NEError::file_couldnt_make(path)),
         Ok(_) => {}
      };
   }

   let file_path = format!("{}{}", path, name);
   let mut file = match fs::File::create(&file_path) {
      Ok(f) => f,
      Err(_) => {
         return NEResult::ER(NEError::File {
            path: file_path,
            kind: NEFileErrKind::CouldNotMake,
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

pub(crate) fn read_as_bytes(path: &str) -> NEResult<Vec<u8>> {
   let mut contents: Vec<u8> = Vec::new();

   let mut err;
   match fs::File::open(&path) {
      Ok(mut file) => match file.read_to_end(&mut contents) {
         Ok(_) => return NEResult::OK(contents),
         Err(e) => err = e,
      },
      Err(e) => {
         err = e;
      }
   }

   let kind = match err.kind() {
      ErrorKind::NotFound | ErrorKind::InvalidInput => NEFileErrKind::NotValid,
      ErrorKind::PermissionDenied => NEFileErrKind::NoPerms,
      _ => NEFileErrKind::Unknown,
   };
   NEResult::ER(NEError::File {
      path: path.to_string(),
      kind,
   })
}

pub(crate) fn read_as_string(path: &str) -> NEResult<String> {
   let mut contents = String::new();

   let mut err;
   match fs::File::open(&path) {
      Ok(mut file) => match file.read_to_string(&mut contents) {
         Ok(_) => return NEResult::OK(contents),
         Err(e) => err = e,
      },
      Err(e) => {
         err = e;
      }
   }

   let kind = match err.kind() {
      ErrorKind::NotFound | ErrorKind::InvalidInput => NEFileErrKind::NotValid,
      ErrorKind::PermissionDenied => NEFileErrKind::NoPerms,
      _ => NEFileErrKind::Unknown,
   };
   NEResult::ER(NEError::File {
      path: path.to_string(),
      kind,
   })
}
