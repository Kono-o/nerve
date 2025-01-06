use crate::asset::obj::NEObjErrKind;
use crate::consts;
use crate::engine::NEInitErrKind;
use std::io::ErrorKind;
use std::{fs, process};

pub(crate) enum NEFileErrKind {
   NoFile,
   NoPerms,
   NotValidPath,
   Unsupported,
   Unknown,
}

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
            process::exit(consts::ERROR_EXIT_CODE)
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

pub enum NEError {
   Init { kind: NEInitErrKind },
   File { kind: NEFileErrKind, path: String },
   Obj { kind: NEObjErrKind, path: String },
}

impl NEError {
   pub fn msg(&self) -> String {
      let msg = match self {
         NEError::Init { kind } => {
            let kind_msg = match kind {
               NEInitErrKind::GlfwInit => "glfw init failed",
               NEInitErrKind::APIUnavailable(api) => &format!("{api} is unavailable"),
               NEInitErrKind::APIWrongVersion(api) => &format!("{api} not a real version"),
               NEInitErrKind::NoMonitor => "no monitor found",
               NEInitErrKind::NotVidMode => "no vid mode found",
               NEInitErrKind::WindowHasNoContext => "window has no context",
               NEInitErrKind::CouldNotMakeWindow => "could not make window",
               NEInitErrKind::Unknown(desc) => &format!("unknown error {desc}"),
            };
            format!("(init) -> {kind_msg}")
         }
         NEError::File { kind, path } => {
            let kind_msg = match kind {
               NEFileErrKind::NoFile => "does not exist",
               NEFileErrKind::NoPerms => "no permissions",
               NEFileErrKind::NotValidPath => "path not valid",
               NEFileErrKind::Unsupported => "unsupported type",
               NEFileErrKind::Unknown => "unknown error",
            };
            format!("(file) -> {kind_msg} [{path}]")
         }
         NEError::Obj { kind, path } => {
            let kind_msg = match kind {
               NEObjErrKind::NonTriMesh => "not triangulated!",
            };
            format!("(obj) -> {kind_msg} [{path}]")
         }
      };
      format!("NERVE ERROR: {msg}")
   }
   pub fn log(&self) {
      let msg = self.msg();
      println!("{msg}");
   }
   pub fn log_and_exit(&self) {
      self.log();
      process::exit(consts::ERROR_EXIT_CODE);
   }
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
