
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the username of a supergroup or channel, requires creator privileges in the supergroup or channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSupergroupUsername {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setSupergroupUsername
  /// Identifier of the supergroup or channel.
  supergroup_id: Option<i32>,
  /// New value of the username. Use an empty string to remove the username.
  username: Option<String>,
  
}



impl Object for SetSupergroupUsername {}
impl RObject for SetSupergroupUsername {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setSupergroupUsername" }
  fn td_type(&self) -> RTDType { RTDType::SetSupergroupUsername }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetSupergroupUsername {}


impl SetSupergroupUsername {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setSupergroupUsername".to_string(),
      supergroup_id: None,
      username: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn username(&self) -> Option<String> { self.username.clone() }
  #[doc(hidden)] pub fn _set_username(&mut self, username: String) -> &mut Self { self.username = Some(username); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



