
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a public HTTPS link to a message. Available only for messages in public supergroups and channels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPublicMessageLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPublicMessageLink
  /// Identifier of the chat to which the message belongs.
  chat_id: Option<i64>,
  /// Identifier of the message.
  message_id: Option<i64>,
  /// Pass true if a link for a whole media album should be returned.
  for_album: Option<bool>,
  
}



impl Object for GetPublicMessageLink {}
impl RObject for GetPublicMessageLink {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPublicMessageLink" }
  fn td_type(&self) -> RTDType { RTDType::GetPublicMessageLink }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPublicMessageLink {}


impl GetPublicMessageLink {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPublicMessageLink".to_string(),
      chat_id: None,
      message_id: None,
      for_album: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn for_album(&self) -> Option<bool> { self.for_album.clone() }
  #[doc(hidden)] pub fn _set_for_album(&mut self, for_album: bool) -> &mut Self { self.for_album = Some(for_album); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



