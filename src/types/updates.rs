
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of updates. 
#[derive(Debug, Serialize, Deserialize)]
pub struct Updates {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updates
  /// List of updates.
  updates: Option<Vec<Box<Update>>>,
  
}


impl Clone for Updates {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for Updates {}
impl RObject for Updates {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updates" }
  fn td_type(&self) -> RTDType { RTDType::Updates }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Updates {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updates".to_string(),
      updates: None,
      
    }
  }
  
  pub fn updates(&self) -> Option<Vec<Box<Update>>> { self.updates.clone() }
  #[doc(hidden)] pub fn _set_updates(&mut self, updates: Vec<Box<Update>>) -> &mut Self { self.updates = Some(updates); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



