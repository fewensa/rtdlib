
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A deleted chat photo. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChatDeletePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageChatDeletePhoto
  
}



impl Object for MessageChatDeletePhoto {}
impl RObject for MessageChatDeletePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageChatDeletePhoto" }
  fn td_type(&self) -> RTDType { RTDType::MessageChatDeletePhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageChatDeletePhoto {}


impl MessageChatDeletePhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageChatDeletePhoto".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



