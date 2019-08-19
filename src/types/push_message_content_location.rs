
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a location. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentLocation
  /// True, if the location is live.
  is_live: Option<bool>,
  /// True, if the message is a pinned message with the specified content.
  is_pinned: Option<bool>,
  
}



impl Object for PushMessageContentLocation {}
impl RObject for PushMessageContentLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentLocation" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentLocation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentLocation {}


impl PushMessageContentLocation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentLocation".to_string(),
      is_live: None,
      is_pinned: None,
      
    }
  }
  
  pub fn is_live(&self) -> Option<bool> { self.is_live.clone() }
  #[doc(hidden)] pub fn _set_is_live(&mut self, is_live: bool) -> &mut Self { self.is_live = Some(is_live); self }
  
  pub fn is_pinned(&self) -> Option<bool> { self.is_pinned.clone() }
  #[doc(hidden)] pub fn _set_is_pinned(&mut self, is_pinned: bool) -> &mut Self { self.is_pinned = Some(is_pinned); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



