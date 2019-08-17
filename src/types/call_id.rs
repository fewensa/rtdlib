
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains the call identifier. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // callId
  /// Call identifier.
  id: Option<i32>,
  
}



impl Object for CallId {}
impl RObject for CallId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "callId" }
  fn td_type(&self) -> RTDType { RTDType::CallId }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl CallId {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "callId".to_string(),
      id: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i32> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i32) -> &mut Self { self.id = Some(id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



