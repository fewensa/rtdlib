
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some data of a basic group has changed. This update is guaranteed to come before the basic group identifier is returned to the client. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBasicGroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateBasicGroup
  /// New data about the group.
  basic_group: Option<BasicGroup>,
  
}



impl Object for UpdateBasicGroup {}
impl RObject for UpdateBasicGroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateBasicGroup" }
  fn td_type(&self) -> RTDType { RTDType::UpdateBasicGroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateBasicGroup {}


impl UpdateBasicGroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateBasicGroup".to_string(),
      basic_group: None,
      
    }
  }
  
  pub fn basic_group(&self) -> Option<BasicGroup> { self.basic_group.clone() }
  #[doc(hidden)] pub fn _set_basic_group(&mut self, basic_group: BasicGroup) -> &mut Self { self.basic_group = Some(basic_group); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



