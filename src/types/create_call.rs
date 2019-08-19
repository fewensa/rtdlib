
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Creates a new call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createCall
  /// Identifier of the user to be called.
  user_id: Option<i32>,
  /// Description of the call protocols supported by the client.
  protocol: Option<CallProtocol>,
  
}



impl Object for CreateCall {}
impl RObject for CreateCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createCall" }
  fn td_type(&self) -> RTDType { RTDType::CreateCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateCall {}


impl CreateCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createCall".to_string(),
      user_id: None,
      protocol: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn protocol(&self) -> Option<CallProtocol> { self.protocol.clone() }
  #[doc(hidden)] pub fn _set_protocol(&mut self, protocol: CallProtocol) -> &mut Self { self.protocol = Some(protocol); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



