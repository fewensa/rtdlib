
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A supergroup (i.e. a chat with up to GetOption("supergroup_max_size") other users), or channel (with unlimited members). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatTypeSupergroup
  /// Supergroup or channel identifier.
  supergroup_id: Option<i32>,
  /// True, if the supergroup is a channel.
  is_channel: Option<bool>,
  
}



impl Object for ChatTypeSupergroup {}
impl RObject for ChatTypeSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatTypeSupergroup" }
  fn td_type(&self) -> RTDType { RTDType::ChatTypeSupergroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatType for ChatTypeSupergroup {}


impl ChatTypeSupergroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatTypeSupergroup".to_string(),
      supergroup_id: None,
      is_channel: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn is_channel(&self) -> Option<bool> { self.is_channel.clone() }
  #[doc(hidden)] pub fn _set_is_channel(&mut self, is_channel: bool) -> &mut Self { self.is_channel = Some(is_channel); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



