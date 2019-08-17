
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a globally unique push notification subscription identifier for identification of an account, which has received a push notification. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPushReceiverId {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getPushReceiverId
  /// JSON-encoded push notification payload.
  payload: Option<String>,
  
}



impl Object for GetPushReceiverId {}
impl RObject for GetPushReceiverId {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getPushReceiverId" }
  fn td_type(&self) -> RTDType { RTDType::GetPushReceiverId }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetPushReceiverId {}


impl GetPushReceiverId {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getPushReceiverId".to_string(),
      payload: None,
      
    }
  }
  
  pub fn payload(&self) -> Option<String> { self.payload.clone() }
  #[doc(hidden)] pub fn _set_payload(&mut self, payload: String) -> &mut Self { self.payload = Some(payload); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



