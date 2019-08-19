
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a supergroup or channel by its identifier. This is an offline request if the current user is not a bot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getSupergroup
  /// Supergroup or channel identifier.
  supergroup_id: Option<i32>,
  
}



impl Object for GetSupergroup {}
impl RObject for GetSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSupergroup" }
  fn td_type(&self) -> RTDType { RTDType::GetSupergroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetSupergroup {}


impl GetSupergroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getSupergroup".to_string(),
      supergroup_id: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i32> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i32) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



