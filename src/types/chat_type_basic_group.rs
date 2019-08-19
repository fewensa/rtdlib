
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A basic group (i.e., a chat with 0-200 other users). 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTypeBasicGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatTypeBasicGroup
  /// Basic group identifier.
  basic_group_id: Option<i32>,
  
}



impl Object for ChatTypeBasicGroup {}
impl RObject for ChatTypeBasicGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatTypeBasicGroup" }
  fn td_type(&self) -> RTDType { RTDType::ChatTypeBasicGroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatType for ChatTypeBasicGroup {}


impl ChatTypeBasicGroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatTypeBasicGroup".to_string(),
      basic_group_id: None,
      
    }
  }
  
  pub fn basic_group_id(&self) -> Option<i32> { self.basic_group_id.clone() }
  #[doc(hidden)] pub fn _set_basic_group_id(&mut self, basic_group_id: i32) -> &mut Self { self.basic_group_id = Some(basic_group_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



