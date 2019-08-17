
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents information about a venue. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultVenue
  /// Unique identifier of the query result.
  id: Option<String>,
  /// Venue result.
  venue: Option<Venue>,
  /// Result thumbnail; may be null.
  thumbnail: Option<PhotoSize>,
  
}



impl Object for InlineQueryResultVenue {}
impl RObject for InlineQueryResultVenue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultVenue" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultVenue }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultVenue {}


impl InlineQueryResultVenue {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultVenue".to_string(),
      id: None,
      venue: None,
      thumbnail: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn venue(&self) -> Option<Venue> { self.venue.clone() }
  #[doc(hidden)] pub fn _set_venue(&mut self, venue: Venue) -> &mut Self { self.venue = Some(venue); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



