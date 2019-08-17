
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a point on the map. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultLocation
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Location result.
  location: Option<Location>,
  /// Title of the result.
  title: Option<String>,
  /// Result thumbnail; may be null.
  thumbnail: Option<PhotoSize>,
  
}



impl Object for InlineQueryResultLocation {}
impl RObject for InlineQueryResultLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultLocation" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultLocation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultLocation {}


impl InlineQueryResultLocation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultLocation".to_string(),
      id: None,
      location: None,
      title: None,
      thumbnail: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn location(&self) -> Option<Location> { self.location.clone() }
  #[doc(hidden)] pub fn _set_location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



