
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An authentication code is delivered via a private Telegram message, which can be viewed in another client. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationCodeTypeTelegramMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // authenticationCodeTypeTelegramMessage
  /// Length of the code.
  length: Option<i32>,
  
}



impl Object for AuthenticationCodeTypeTelegramMessage {}
impl RObject for AuthenticationCodeTypeTelegramMessage {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "authenticationCodeTypeTelegramMessage" }
  fn td_type(&self) -> RTDType { RTDType::AuthenticationCodeTypeTelegramMessage }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl AuthenticationCodeType for AuthenticationCodeTypeTelegramMessage {}


impl AuthenticationCodeTypeTelegramMessage {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "authenticationCodeTypeTelegramMessage".to_string(),
      length: None,
      
    }
  }
  
  pub fn length(&self) -> Option<i32> { self.length.clone() }
  #[doc(hidden)] pub fn _set_length(&mut self, length: i32) -> &mut Self { self.length = Some(length); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



