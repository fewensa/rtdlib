
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the high scores for a game and some part of the high score table in the range of the specified user; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGameHighScores {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getGameHighScores
  /// The chat that contains the message with the game.
  chat_id: Option<i64>,
  /// Identifier of the message.
  message_id: Option<i64>,
  /// User identifier.
  user_id: Option<i32>,
  
}



impl Object for GetGameHighScores {}
impl RObject for GetGameHighScores {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getGameHighScores" }
  fn td_type(&self) -> RTDType { RTDType::GetGameHighScores }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetGameHighScores {}


impl GetGameHighScores {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getGameHighScores".to_string(),
      chat_id: None,
      message_id: None,
      user_id: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn message_id(&self) -> Option<i64> { self.message_id.clone() }
  #[doc(hidden)] pub fn _set_message_id(&mut self, message_id: i64) -> &mut Self { self.message_id = Some(message_id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



