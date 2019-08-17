
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Instructs clients to remove the keyboard once this message has been received. This kind of keyboard can't be received in an incoming message; instead, UpdateChatReplyMarkup with message_id == 0 will be sent. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyMarkupRemoveKeyboard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // replyMarkupRemoveKeyboard
  /// True, if the keyboard is removed only for the mentioned users or the target user of a reply.
  is_personal: Option<bool>,
  
}



impl Object for ReplyMarkupRemoveKeyboard {}
impl RObject for ReplyMarkupRemoveKeyboard {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "replyMarkupRemoveKeyboard" }
  fn td_type(&self) -> RTDType { RTDType::ReplyMarkupRemoveKeyboard }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ReplyMarkup for ReplyMarkupRemoveKeyboard {}


impl ReplyMarkupRemoveKeyboard {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "replyMarkupRemoveKeyboard".to_string(),
      is_personal: None,
      
    }
  }
  
  pub fn is_personal(&self) -> Option<bool> { self.is_personal.clone() }
  #[doc(hidden)] pub fn _set_is_personal(&mut self, is_personal: bool) -> &mut Self { self.is_personal = Some(is_personal); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



