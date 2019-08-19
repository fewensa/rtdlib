
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A forwarded message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessageForwarded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageForwarded
  /// Identifier for the chat this forwarded message came from.
  from_chat_id: Option<i64>,
  /// Identifier of the message to forward.
  message_id: Option<i64>,
  /// True, if a game message should be shared within a launched game; applies only to game messages.
  in_game_share: Option<bool>,
  
}



impl Object for InputMessageForwarded {}
impl RObject for InputMessageForwarded {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageForwarded" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageForwarded }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageForwarded {}


impl InputMessageForwarded {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageForwarded".to_string(),
      from_chat_id: None,
      message_id: None,
      in_game_share: None,
      
    }
  }
  
  pub fn from_chat_id(&self) -> Option<i64> { self.from_chat_id.clone() }
  #[doc(hidden)] pub fn _set_from_chat_id(&mut self, from_chat_id: i64) -> &mut Self { self.from_chat_id = Some(from_chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn in_game_share(&self) -> Option<bool> { self.in_game_share.clone() }
  #[doc(hidden)] pub fn _set_in_game_share(&mut self, in_game_share: bool) -> &mut Self { self.in_game_share = Some(in_game_share); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



