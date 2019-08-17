
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Some data of a supergroup or a channel has changed. This update is guaranteed to come before the supergroup identifier is returned to the client. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateSupergroup
  /// New data about the supergroup.
  supergroup: Option<Supergroup>,
  
}



impl Object for UpdateSupergroup {}
impl RObject for UpdateSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateSupergroup" }
  fn td_type(&self) -> RTDType { RTDType::UpdateSupergroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateSupergroup {}


impl UpdateSupergroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateSupergroup".to_string(),
      supergroup: None,
      
    }
  }
  
  pub fn supergroup(&self) -> Option<Supergroup> { self.supergroup.clone() }
  #[doc(hidden)] pub fn _set_supergroup(&mut self, supergroup: Supergroup) -> &mut Self { self.supergroup = Some(supergroup); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



