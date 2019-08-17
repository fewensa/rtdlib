
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns saved animations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSavedAnimations {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getSavedAnimations
  
}



impl Object for GetSavedAnimations {}
impl RObject for GetSavedAnimations {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getSavedAnimations" }
  fn td_type(&self) -> RTDType { RTDType::GetSavedAnimations }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetSavedAnimations {}


impl GetSavedAnimations {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getSavedAnimations".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



