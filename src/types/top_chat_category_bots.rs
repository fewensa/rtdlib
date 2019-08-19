
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A category containing frequently used private chats with bot users. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopChatCategoryBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // topChatCategoryBots
  
}



impl Object for TopChatCategoryBots {}
impl RObject for TopChatCategoryBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryBots" }
  fn td_type(&self) -> RTDType { RTDType::TopChatCategoryBots }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TopChatCategory for TopChatCategoryBots {}


impl TopChatCategoryBots {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "topChatCategoryBots".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



