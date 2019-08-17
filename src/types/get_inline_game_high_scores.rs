
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns game high scores and some part of the high score table in the range of the specified user; for bots only.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInlineGameHighScores {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getInlineGameHighScores
  /// Inline message identifier.
  inline_message_id: Option<String>,
  /// User identifier.
  user_id: Option<i32>,
  
}



impl Object for GetInlineGameHighScores {}
impl RObject for GetInlineGameHighScores {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInlineGameHighScores" }
  fn td_type(&self) -> RTDType { RTDType::GetInlineGameHighScores }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetInlineGameHighScores {}


impl GetInlineGameHighScores {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getInlineGameHighScores".to_string(),
      inline_message_id: None,
      user_id: None,
      
    }
  }
  
  pub fn inline_message_id(&self) -> Option<String> { self.inline_message_id.clone() }
  #[doc(hidden)] pub fn _set_inline_message_id(&mut self, inline_message_id: String) -> &mut Self { self.inline_message_id = Some(inline_message_id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



