
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A category containing frequently used chats with inline bots sorted by their usage in inline mode. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopChatCategoryInlineBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // topChatCategoryInlineBots
  
}



impl Object for TopChatCategoryInlineBots {}
impl RObject for TopChatCategoryInlineBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryInlineBots" }
  fn td_type(&self) -> RTDType { RTDType::TopChatCategoryInlineBots }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TopChatCategory for TopChatCategoryInlineBots {}


impl TopChatCategoryInlineBots {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "topChatCategoryInlineBots".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



