
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A general message with hidden content. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentHidden {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentHidden
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentHidden {}
impl RObject for PushMessageContentHidden {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentHidden" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentHidden }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentHidden {}


impl PushMessageContentHidden {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentHidden".to_string(),
      is_pinned: None,
      
    }
  }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



