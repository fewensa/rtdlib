
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a user contact. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentContact {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentContact
  /// Contact's name.
  name: Option<String>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentContact {}
impl RObject for PushMessageContentContact {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentContact" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentContact }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentContact {}


impl PushMessageContentContact {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentContact".to_string(),
      name: None,
      is_pinned: None,
      
    }
  }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



