
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A map. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageBlockMap {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pageBlockMap
  /// Location of the map center.
  location: Option<Location>,
  /// Map zoom level.
  zoom: Option<i32>,
  /// Map width.
  width: Option<i32>,
  /// Map height.
  height: Option<i32>,
  /// Block caption.
  caption: Option<PageBlockCaption>,
  
}



impl Object for PageBlockMap {}
impl RObject for PageBlockMap {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pageBlockMap" }
  fn td_type(&self) -> RTDType { RTDType::PageBlockMap }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PageBlock for PageBlockMap {}


impl PageBlockMap {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pageBlockMap".to_string(),
      location: None,
      zoom: None,
      width: None,
      height: None,
      caption: None,
      
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
  
  pub fn caption(&self) -> Option<PageBlockCaption> { self.caption.clone() }
  #[doc(hidden)] pub fn _set_caption(&mut self, caption: PageBlockCaption) -> &mut Self { self.caption = Some(caption); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



