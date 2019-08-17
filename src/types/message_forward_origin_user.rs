
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The message was originally written by a known user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageForwardOriginUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageForwardOriginUser
  /// Identifier of the user that originally sent the message.
  sender_user_id: Option<i32>,
  
}



impl Object for MessageForwardOriginUser {}
impl RObject for MessageForwardOriginUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardOriginUser" }
  fn td_type(&self) -> RTDType { RTDType::MessageForwardOriginUser }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageForwardOrigin for MessageForwardOriginUser {}


impl MessageForwardOriginUser {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageForwardOriginUser".to_string(),
      sender_user_id: None,
      
    }
  }
  
  pub fn sender_user_id(&self) -> Option<i32> { self.sender_user_id.clone() }
  #[doc(hidden)] pub fn _set_sender_user_id(&mut self, sender_user_id: i32) -> &mut Self { self.sender_user_id = Some(sender_user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



