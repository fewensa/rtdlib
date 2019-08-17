
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes user answer to a poll.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPollAnswer {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setPollAnswer
  /// Identifier of the chat to which the poll belongs.
  chat_id: Option<i64>,
  /// Identifier of the message containing the poll.
  message_id: Option<i64>,
  /// 0-based identifiers of options, chosen by the user. Currently user can't choose more than 1 option.
  option_ids: Option<Vec<i32>>,
  
}



impl Object for SetPollAnswer {}
impl RObject for SetPollAnswer {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setPollAnswer" }
  fn td_type(&self) -> RTDType { RTDType::SetPollAnswer }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetPollAnswer {}


impl SetPollAnswer {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setPollAnswer".to_string(),
      chat_id: None,
      message_id: None,
      option_ids: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn option_ids(&self) -> Option<Vec<i32>> { self.option_ids.clone() }
  #[doc(hidden)] pub fn _set_option_ids(&mut self, option_ids: Vec<i32>) -> &mut Self { self.option_ids = Some(option_ids); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



