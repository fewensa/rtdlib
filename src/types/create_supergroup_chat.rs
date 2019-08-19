
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an existing chat corresponding to a known supergroup or channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSupergroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createSupergroupChat
  /// Supergroup or channel identifier.
  supergroup_id: Option<i32>,
  /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect.
  force: Option<bool>,
  
}



impl Object for CreateSupergroupChat {}
impl RObject for CreateSupergroupChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createSupergroupChat" }
  fn td_type(&self) -> RTDType { RTDType::CreateSupergroupChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateSupergroupChat {}


impl CreateSupergroupChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createSupergroupChat".to_string(),
      supergroup_id: None,
      force: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn force(&self) -> Option<bool> { self.force.clone() }
  #[doc(hidden)] pub fn _set_force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



