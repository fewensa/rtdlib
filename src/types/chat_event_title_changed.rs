
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat title was changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventTitleChanged {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventTitleChanged
  /// Previous chat title.
  old_title: Option<String>,
  /// New chat title.
  new_title: Option<String>,
  
}



impl Object for ChatEventTitleChanged {}
impl RObject for ChatEventTitleChanged {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventTitleChanged" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventTitleChanged }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventTitleChanged {}


impl ChatEventTitleChanged {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventTitleChanged".to_string(),
      old_title: None,
      new_title: None,
      
    }
  }
  
  pub fn old_title(&self) -> Option<String> { self.old_title.clone() }
  #[doc(hidden)] pub fn _set_old_title(&mut self, old_title: String) -> &mut Self { self.old_title = Some(old_title); self }
  
  pub fn new_title(&self) -> Option<String> { self.new_title.clone() }
  #[doc(hidden)] pub fn _set_new_title(&mut self, new_title: String) -> &mut Self { self.new_title = Some(new_title); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



