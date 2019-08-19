
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultPhoto
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Photo.
  photo: Option<Photo>,
  /// Title of the result, if known.
  title: Option<String>,
  /// A short description of the result, if known.
  description: Option<String>,
  
}



impl Object for InlineQueryResultPhoto {}
impl RObject for InlineQueryResultPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultPhoto" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultPhoto {}


impl InlineQueryResultPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultPhoto".to_string(),
      id: None,
      photo: None,
      title: None,
      description: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn photo(&self) -> Option<Photo> { self.photo.clone() }
  #[doc(hidden)] pub fn _set_photo(&mut self, photo: Photo) -> &mut Self { self.photo = Some(photo); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



