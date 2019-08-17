
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Checks the authentication token of a bot; to log in as a bot. Works only when the current authorization state is 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckAuthenticationBotToken {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // checkAuthenticationBotToken
  /// The bot token.
  token: Option<String>,
  
}



impl Object for CheckAuthenticationBotToken {}
impl RObject for CheckAuthenticationBotToken {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "checkAuthenticationBotToken" }
  fn td_type(&self) -> RTDType { RTDType::CheckAuthenticationBotToken }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CheckAuthenticationBotToken {}


impl CheckAuthenticationBotToken {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "checkAuthenticationBotToken".to_string(),
      token: None,
      
    }
  }
  
  pub fn token(&self) -> Option<String> { self.token.clone() }
  #[doc(hidden)] pub fn _set_token(&mut self, token: String) -> &mut Self { self.token = Some(token); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



