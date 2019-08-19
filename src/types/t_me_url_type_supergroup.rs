
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A URL linking to a public supergroup or channel. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMeUrlTypeSupergroup {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // tMeUrlTypeSupergroup
  /// Identifier of the supergroup or channel.
  supergroup_id: Option<i64>,
  
}



impl Object for TMeUrlTypeSupergroup {}
impl RObject for TMeUrlTypeSupergroup {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrlTypeSupergroup" }
  fn td_type(&self) -> RTDType { RTDType::TMeUrlTypeSupergroup }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TMeUrlType for TMeUrlTypeSupergroup {}


impl TMeUrlTypeSupergroup {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "tMeUrlTypeSupergroup".to_string(),
      supergroup_id: None,
      
    }
  }
  
  pub fn supergroup_id(&self) -> Option<i64> { self.supergroup_id.clone() }
  #[doc(hidden)] pub fn _set_supergroup_id(&mut self, supergroup_id: i64) -> &mut Self { self.supergroup_id = Some(supergroup_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



