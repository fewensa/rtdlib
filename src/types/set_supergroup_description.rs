
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes information about a supergroup or channel; requires appropriate administrator rights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSupergroupDescription {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setSupergroupDescription
  /// Identifier of the supergroup or channel.
  supergroup_id: Option<i32>,
  /// New supergroup or channel description; 0-255 characters.
  description: Option<String>,
  
}



impl Object for SetSupergroupDescription {}
impl RObject for SetSupergroupDescription {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setSupergroupDescription" }
  fn td_type(&self) -> RTDType { RTDType::SetSupergroupDescription }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetSupergroupDescription {}


impl SetSupergroupDescription {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setSupergroupDescription".to_string(),
      supergroup_id: None,
      description: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn description(&self) -> Option<String> { self.description.clone() }
  #[doc(hidden)] pub fn _set_description(&mut self, description: String) -> &mut Self { self.description = Some(description); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



