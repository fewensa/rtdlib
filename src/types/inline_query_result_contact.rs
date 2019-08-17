
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a user contact. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQueryResultContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineQueryResultContact
  /// Unique identifier of the query result.
  id: Option<String>,
  /// A user contact.
  contact: Option<Contact>,
  /// Result thumbnail; may be null.
  thumbnail: Option<PhotoSize>,
  
}



impl Object for InlineQueryResultContact {}
impl RObject for InlineQueryResultContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineQueryResultContact" }
  fn td_type(&self) -> RTDType { RTDType::InlineQueryResultContact }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineQueryResult for InlineQueryResultContact {}


impl InlineQueryResultContact {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineQueryResultContact".to_string(),
      id: None,
      contact: None,
      thumbnail: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn contact(&self) -> Option<Contact> { self.contact.clone() }
  #[doc(hidden)] pub fn _set_contact(&mut self, contact: Contact) -> &mut Self { self.contact = Some(contact); self }
  
  pub fn thumbnail(&self) -> Option<PhotoSize> { self.thumbnail.clone() }
  #[doc(hidden)] pub fn _set_thumbnail(&mut self, thumbnail: PhotoSize) -> &mut Self { self.thumbnail = Some(thumbnail); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



