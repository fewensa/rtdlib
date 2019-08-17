
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A category containing frequently used basic groups and supergroups. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopChatCategoryGroups {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // topChatCategoryGroups
  
}



impl Object for TopChatCategoryGroups {}
impl RObject for TopChatCategoryGroups {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryGroups" }
  fn td_type(&self) -> RTDType { RTDType::TopChatCategoryGroups }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TopChatCategory for TopChatCategoryGroups {}


impl TopChatCategoryGroups {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "topChatCategoryGroups".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



