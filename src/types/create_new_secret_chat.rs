
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Creates a new secret chat. Returns the newly created chat.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNewSecretChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // createNewSecretChat
  /// Identifier of the target user.
  user_id: Option<i32>,
  
}



impl Object for CreateNewSecretChat {}
impl RObject for CreateNewSecretChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "createNewSecretChat" }
  fn td_type(&self) -> RTDType { RTDType::CreateNewSecretChat }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for CreateNewSecretChat {}


impl CreateNewSecretChat {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "createNewSecretChat".to_string(),
      user_id: None,
      
    }
  }
  
  pub fn user_id(&self) -> Option<i32> { self.user_id.clone() }
  #[doc(hidden)] pub fn _set_user_id(&mut self, user_id: i32) -> &mut Self { self.user_id = Some(user_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



