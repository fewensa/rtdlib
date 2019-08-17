
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits the content of a live location in an inline message sent via a bot; for bots only.
#[derive(Debug, Serialize, Deserialize)]
pub struct EditInlineMessageLiveLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editInlineMessageLiveLocation
  /// Inline message identifier.
  inline_message_id: Option<String>,
  /// The new message reply markup.
  reply_markup: Option<Box<ReplyMarkup>>,
  /// New location content of the message; may be null. Pass null to stop sharing the live location.
  location: Option<Location>,
  
}


impl Clone for EditInlineMessageLiveLocation {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EditInlineMessageLiveLocation {}
impl RObject for EditInlineMessageLiveLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editInlineMessageLiveLocation" }
  fn td_type(&self) -> RTDType { RTDType::EditInlineMessageLiveLocation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditInlineMessageLiveLocation {}


impl EditInlineMessageLiveLocation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editInlineMessageLiveLocation".to_string(),
      inline_message_id: None,
      reply_markup: None,
      location: None,
      
    }
  }
  
  pub fn inline_message_id(&self) -> Option<String> { self.inline_message_id.clone() }
  #[doc(hidden)] pub fn _set_inline_message_id(&mut self, inline_message_id: String) -> &mut Self { self.inline_message_id = Some(inline_message_id); self }
  
  pub fn reply_markup(&self) -> Option<Box<ReplyMarkup>> { self.reply_markup.clone() }
  #[doc(hidden)] pub fn _set_reply_markup(&mut self, reply_markup: Box<ReplyMarkup>) -> &mut Self { self.reply_markup = Some(reply_markup); self }
  
  pub fn location(&self) -> Option<Location> { self.location.clone() }
  #[doc(hidden)] pub fn _set_location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



