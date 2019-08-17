
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of wallpapers. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallpapers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // wallpapers
  /// A list of wallpapers.
  wallpapers: Option<Vec<Wallpaper>>,
  
}



impl Object for Wallpapers {}
impl RObject for Wallpapers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "wallpapers" }
  fn td_type(&self) -> RTDType { RTDType::Wallpapers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Wallpapers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "wallpapers".to_string(),
      wallpapers: None,
      
    }
  }
  
  pub fn wallpapers(&self) -> Option<Vec<Wallpaper>> { self.wallpapers.clone() }
  #[doc(hidden)] pub fn _set_wallpapers(&mut self, wallpapers: Vec<Wallpaper>) -> &mut Self { self.wallpapers = Some(wallpapers); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



