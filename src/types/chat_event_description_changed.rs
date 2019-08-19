
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat description was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventDescriptionChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventDescriptionChanged
  /// Previous chat description.
  old_description: Option<String>,
  /// New chat description.
  new_description: Option<String>,
  
}



impl Object for ChatEventDescriptionChanged {}
impl RObject for ChatEventDescriptionChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventDescriptionChanged" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventDescriptionChanged }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventDescriptionChanged {}


impl ChatEventDescriptionChanged {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventDescriptionChanged".to_string(),
      old_description: None,
      new_description: None,
      
    }
  }
  
  pub fn old_description(&self) -> Option<String> { self.old_description.clone() }
  #[doc(hidden)] pub fn _set_old_description(&mut self, old_description: String) -> &mut Self { self.old_description = Some(old_description); self }
  
  pub fn new_description(&self) -> Option<String> { self.new_description.clone() }
  #[doc(hidden)] pub fn _set_new_description(&mut self, new_description: String) -> &mut Self { self.new_description = Some(new_description); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



