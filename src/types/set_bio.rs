
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Changes the bio of the current user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetBio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setBio
  /// The new value of the user bio; 0-70 characters without line feeds.
  bio: Option<String>,
  
}



impl Object for SetBio {}
impl RObject for SetBio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setBio" }
  fn td_type(&self) -> RTDType { RTDType::SetBio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetBio {}


impl SetBio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setBio".to_string(),
      bio: None,
      
    }
  }
  
  pub fn bio(&self) -> Option<String> { self.bio.clone() }
  #[doc(hidden)] pub fn _set_bio(&mut self, bio: String) -> &mut Self { self.bio = Some(bio); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



