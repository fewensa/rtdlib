
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an existing chat corresponding to a given user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePrivateChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createPrivateChat
  /// User identifier.
  user_id: Option<i32>,
  /// If true, the chat will be created without network request. In this case all information about the chat except its type, title and photo can be incorrect.
  force: Option<bool>,
  
}



impl Object for CreatePrivateChat {}
impl RObject for CreatePrivateChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createPrivateChat" }
  fn td_type(&self) -> RTDType { RTDType::CreatePrivateChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreatePrivateChat {}


impl CreatePrivateChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createPrivateChat".to_string(),
      user_id: None,
      force: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn force(&self) -> Option<bool> { self.force.clone() }
  #[doc(hidden)] pub fn _set_force(&mut self, force: bool) -> &mut Self { self.force = Some(force); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



