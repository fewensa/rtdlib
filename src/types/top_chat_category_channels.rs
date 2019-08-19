
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A category containing frequently used channels. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopChatCategoryChannels {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // topChatCategoryChannels
  
}



impl Object for TopChatCategoryChannels {}
impl RObject for TopChatCategoryChannels {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "topChatCategoryChannels" }
  fn td_type(&self) -> RTDType { RTDType::TopChatCategoryChannels }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TopChatCategory for TopChatCategoryChannels {}


impl TopChatCategoryChannels {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "topChatCategoryChannels".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



