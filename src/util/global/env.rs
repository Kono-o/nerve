pub(crate) const GLSL_VALIDATOR: &str = "glslangValidator";
const DEFAULT_GLSL_VALIDATOR_PATH: &str = "/media/kono/HDD/apps/vulkan/1.3.296.0/x86_64/bin/";
const DEFAULT_ASSETS_PATH: &str = "assets/";

use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
   static ref GLSL_VALIDATOR_PATH: RwLock<String> =
      RwLock::new(format!("{}{}", DEFAULT_GLSL_VALIDATOR_PATH, GLSL_VALIDATOR));
}

lazy_static! {
   static ref ASSETS_PATH: RwLock<String> = RwLock::new(DEFAULT_ASSETS_PATH.to_string());
}

pub fn set_glsl_validator_path(path: &str) {
   let glv_path = match path.ends_with(GLSL_VALIDATOR) {
      false => match path.ends_with("/") {
         true => {
            format!("{path}{}", GLSL_VALIDATOR)
         }
         false => {
            format!("{path}/{}", GLSL_VALIDATOR)
         }
      },
      true => path.to_string(),
   };
   let mut path_lock = GLSL_VALIDATOR_PATH.write().unwrap();
   *path_lock = glv_path;
}
pub fn glsl_validator_path() -> String {
   GLSL_VALIDATOR_PATH.read().unwrap().clone()
}

pub fn set_assets_path(path: &str) {
   let assets_path = match path.ends_with("/") {
      false => format!("{path}/"),
      true => path.to_string(),
   };
   let mut path_lock = ASSETS_PATH.write().unwrap();
   *path_lock = assets_path;
}
pub fn assets_path() -> String {
   ASSETS_PATH.read().unwrap().clone()
}

pub(crate) fn concat_with_asset(path: &str) -> String {
   let trimmed_path = match path.starts_with("/") {
      true => &path[1..],
      false => path,
   };
   format!("{}{}", assets_path(), trimmed_path)
}
