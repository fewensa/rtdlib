
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user has chosen a result of an inline query; for bots only. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNewChosenInlineResult {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateNewChosenInlineResult
  /// Identifier of the user who sent the query.
  sender_user_id: Option<i32>,
  /// User location, provided by the client; may be null.
  user_location: Option<Location>,
  /// Text of the query.
  query: Option<String>,
  /// Identifier of the chosen result.
  result_id: Option<String>,
  /// Identifier of the sent inline message, if known.
  inline_message_id: Option<String>,
  
}



impl Object for UpdateNewChosenInlineResult {}
impl RObject for UpdateNewChosenInlineResult {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateNewChosenInlineResult" }
  fn td_type(&self) -> RTDType { RTDType::UpdateNewChosenInlineResult }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateNewChosenInlineResult {}


impl UpdateNewChosenInlineResult {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateNewChosenInlineResult".to_string(),
      sender_user_id: None,
      user_location: None,
      query: None,
      result_id: None,
      inline_message_id: None,
      
    }
  }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn user_location(&self) -> Option<Location> { self.user_location.clone() }
  #[doc(hidden)] pub fn _set_user_location(&mut self, user_location: Location) -> &mut Self { self.user_location = Some(user_location); self }
  
  pub fn query(&self) -> Option<String> { self.query.clone() }
  #[doc(hidden)] pub fn _set_query(&mut self, query: String) -> &mut Self { self.query = Some(query); self }
  
  pub fn result_id(&self) -> Option<String> { self.result_id.clone() }
  #[doc(hidden)] pub fn _set_result_id(&mut self, result_id: String) -> &mut Self { self.result_id = Some(result_id); self }
  
  pub fn inline_message_id(&self) -> Option<String> { self.inline_message_id.clone() }
  #[doc(hidden)] pub fn _set_inline_message_id(&mut self, inline_message_id: String) -> &mut Self { self.inline_message_id = Some(inline_message_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



