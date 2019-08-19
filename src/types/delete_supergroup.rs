
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes a supergroup or channel along with all messages in the corresponding chat. This will release the supergroup or channel username and remove all members; requires creator privileges in the supergroup or channel. Chats with more than 1000 members can't be deleted using this method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteSupergroup
  /// Identifier of the supergroup or channel.
  supergroup_id: Option<i32>,
  
}



impl Object for DeleteSupergroup {}
impl RObject for DeleteSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteSupergroup" }
  fn td_type(&self) -> RTDType { RTDType::DeleteSupergroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteSupergroup {}


impl DeleteSupergroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteSupergroup".to_string(),
      supergroup_id: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



