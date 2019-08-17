
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a wallpaper. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallpaper {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // wallpaper
  /// Unique persistent wallpaper identifier.
  id: Option<i32>,
  /// Available variants of the wallpaper in different sizes. These photos can only be downloaded; they can't be sent in a message.
  sizes: Option<Vec<PhotoSize>>,
  /// Main color of the wallpaper in RGB24 format; should be treated as background color if no photos are specified.
  color: Option<i32>,
  
}



impl Object for Wallpaper {}
impl RObject for Wallpaper {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "wallpaper" }
  fn td_type(&self) -> RTDType { RTDType::Wallpaper }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Wallpaper {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "wallpaper".to_string(),
      id: None,
      sizes: None,
      color: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn sizes(&self) -> Option<Vec<PhotoSize>> { self.sizes.clone() }
  #[doc(hidden)] pub fn _set_sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self { self.sizes = Some(sizes); self }
  
  pub fn color(&self) -> Option<i32> { self.color.clone() }
  #[doc(hidden)] pub fn _set_color(&mut self, color: i32) -> &mut Self { self.color = Some(color); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



