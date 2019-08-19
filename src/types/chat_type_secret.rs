
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A secret chat with a user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeSecret {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatTypeSecret
  /// Secret chat identifier.
  secret_chat_id: Option<i32>,
  /// User identifier of the secret chat peer.
  user_id: Option<i32>,
  
}



impl Object for ChatTypeSecret {}
impl RObject for ChatTypeSecret {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatTypeSecret" }
  fn td_type(&self) -> RTDType { RTDType::ChatTypeSecret }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatType for ChatTypeSecret {}


impl ChatTypeSecret {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatTypeSecret".to_string(),
      secret_chat_id: None,
      user_id: None,
      
    }
  }
  
  pub fn secret_chat_id(&self) -> Option<i32> { self.secret_chat_id.clone() }
  #[doc(hidden)] pub fn _set_secret_chat_id(&mut self, secret_chat_id: i32) -> &mut Self { self.secret_chat_id = Some(secret_chat_id); self }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



