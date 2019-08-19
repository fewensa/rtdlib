
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The user is typing a message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatActionTyping {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatActionTyping
  
}



impl Object for ChatActionTyping {}
impl RObject for ChatActionTyping {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatActionTyping" }
  fn td_type(&self) -> RTDType { RTDType::ChatActionTyping }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatAction for ChatActionTyping {}


impl ChatActionTyping {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatActionTyping".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



