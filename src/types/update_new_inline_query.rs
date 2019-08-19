
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A new incoming inline query; for bots only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewInlineQuery {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewInlineQuery
  /// Unique query identifier.
  id: Option<i64>,
  /// Identifier of the user who sent the query.
  sender_user_id: Option<i32>,
  /// User location, provided by the client; may be null.
  user_location: Option<Location>,
  /// Text of the query.
  query: Option<String>,
  /// Offset of the first entry to return.
  offset: Option<String>,
  
}



impl Object for UpdateNewInlineQuery {}
impl RObject for UpdateNewInlineQuery {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewInlineQuery" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewInlineQuery }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewInlineQuery {}


impl UpdateNewInlineQuery {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewInlineQuery".to_string(),
      id: None,
      sender_user_id: None,
      user_location: None,
      query: None,
      offset: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn user_location(&self) -> Option<Location> { self.user_location.clone() }
  #[doc(hidden)] pub fn _set_user_location(&mut self, user_location: Location) -> &mut Self { self.user_location = Some(user_location); self }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn offset(&self) -> Option<String> { self.offset.clone() }
  #[doc(hidden)] pub fn _set_offset(&mut self, offset: String) -> &mut Self { self.offset = Some(offset); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



