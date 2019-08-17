
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a file with a map thumbnail in PNG format. Only map thumbnail files with size less than 1MB can be downloaded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMapThumbnailFile {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getMapThumbnailFile
  /// Location of the map center.
  location: Option<Location>,
  /// Map zoom level; 13-20.
  zoom: Option<i32>,
  /// Map width in pixels before applying scale; 16-1024.
  width: Option<i32>,
  /// Map height in pixels before applying scale; 16-1024.
  height: Option<i32>,
  /// Map scale; 1-3.
  scale: Option<i32>,
  /// Identifier of a chat, in which the thumbnail will be shown. Use 0 if unknown.
  chat_id: Option<i64>,
  
}



impl Object for GetMapThumbnailFile {}
impl RObject for GetMapThumbnailFile {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getMapThumbnailFile" }
  fn td_type(&self) -> RTDType { RTDType::GetMapThumbnailFile }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetMapThumbnailFile {}


impl GetMapThumbnailFile {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getMapThumbnailFile".to_string(),
      location: None,
      zoom: None,
      width: None,
      height: None,
      scale: None,
      chat_id: None,
      
    }
  }
  
  pub fn location(&self) -> Option<Location> { self.location.clone() }
  #[doc(hidden)] pub fn _set_location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  
  pub fn zoom(&self) -> Option<i32> { self.zoom.clone() }
  #[doc(hidden)] pub fn _set_zoom(&mut self, zoom: i32) -> &mut Self { self.zoom = Some(zoom); self }
  
  pub fn width(&self) -> Option<i32> { self.width.clone() }
  #[doc(hidden)] pub fn _set_width(&mut self, width: i32) -> &mut Self { self.width = Some(width); self }
  
  pub fn height(&self) -> Option<i32> { self.height.clone() }
  #[doc(hidden)] pub fn _set_height(&mut self, height: i32) -> &mut Self { self.height = Some(height); self }
  
  pub fn scale(&self) -> Option<i32> { self.scale.clone() }
  #[doc(hidden)] pub fn _set_scale(&mut self, scale: i32) -> &mut Self { self.scale = Some(scale); self }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



