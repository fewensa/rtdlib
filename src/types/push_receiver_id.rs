
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a globally unique push receiver identifier, which can be used to identify which account has received a push notification. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushReceiverId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushReceiverId
  /// The globally unique identifier of push notification subscription.
  id: Option<i64>,
  
}



impl Object for PushReceiverId {}
impl RObject for PushReceiverId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushReceiverId" }
  fn td_type(&self) -> RTDType { RTDType::PushReceiverId }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PushReceiverId {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushReceiverId".to_string(),
      id: None,
      
    }
  }
  
  pub fn id(&self) -> Option<i64> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: i64) -> &mut Self { self.id = Some(id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



