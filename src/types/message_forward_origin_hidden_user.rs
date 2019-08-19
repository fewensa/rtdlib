
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The message was originally written by a user, which is hidden by his privacy settings. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageForwardOriginHiddenUser {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageForwardOriginHiddenUser
  /// Name of the sender.
  sender_name: Option<String>,
  
}



impl Object for MessageForwardOriginHiddenUser {}
impl RObject for MessageForwardOriginHiddenUser {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageForwardOriginHiddenUser" }
  fn td_type(&self) -> RTDType { RTDType::MessageForwardOriginHiddenUser }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageForwardOrigin for MessageForwardOriginHiddenUser {}


impl MessageForwardOriginHiddenUser {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageForwardOriginHiddenUser".to_string(),
      sender_name: None,
      
    }
  }
  
  pub fn sender_name(&self) -> Option<String> { self.sender_name.clone() }
  #[doc(hidden)] pub fn _set_sender_name(&mut self, sender_name: String) -> &mut Self { self.sender_name = Some(sender_name); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



