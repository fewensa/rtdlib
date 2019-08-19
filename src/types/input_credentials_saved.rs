
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Applies if a user chooses some previously saved payment credentials. To use their previously saved credentials, the user must have a valid temporary password. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputCredentialsSaved {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputCredentialsSaved
  /// Identifier of the saved credentials.
  saved_credentials_id: Option<String>,
  
}



impl Object for InputCredentialsSaved {}
impl RObject for InputCredentialsSaved {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputCredentialsSaved" }
  fn td_type(&self) -> RTDType { RTDType::InputCredentialsSaved }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputCredentials for InputCredentialsSaved {}


impl InputCredentialsSaved {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputCredentialsSaved".to_string(),
      saved_credentials_id: None,
      
    }
  }
  
  pub fn saved_credentials_id(&self) -> Option<String> { self.saved_credentials_id.clone() }
  #[doc(hidden)] pub fn _set_saved_credentials_id(&mut self, saved_credentials_id: String) -> &mut Self { self.saved_credentials_id = Some(saved_credentials_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



