
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A category containing frequently used private chats with non-bot users. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopChatCategoryUsers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // topChatCategoryUsers
  
}



impl Object for TopChatCategoryUsers {}
impl RObject for TopChatCategoryUsers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryUsers" }
  fn td_type(&self) -> RTDType { RTDType::TopChatCategoryUsers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TopChatCategory for TopChatCategoryUsers {}


impl TopChatCategoryUsers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "topChatCategoryUsers".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



