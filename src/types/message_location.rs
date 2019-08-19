
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a location. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageLocation
  /// Message content.
  location: Option<Location>,
  /// Time relative to the message sent date until which the location can be updated, in seconds.
  live_period: Option<i32>,
  /// Left time for which the location can be updated, in seconds. updateMessageContent is not sent when this field changes.
  expires_in: Option<i32>,
  
}



impl Object for MessageLocation {}
impl RObject for MessageLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageLocation" }
  fn td_type(&self) -> RTDType { RTDType::MessageLocation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageLocation {}


impl MessageLocation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageLocation".to_string(),
      location: None,
      live_period: None,
      expires_in: None,
      
    }
  }
  
  pub fn location(&self) -> Option<Location> { self.location.clone() }
  #[doc(hidden)] pub fn _set_location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  
  pub fn live_period(&self) -> Option<i32> { self.live_period.clone() }
  #[doc(hidden)] pub fn _set_live_period(&mut self, live_period: i32) -> &mut Self { self.live_period = Some(live_period); self }
  
  pub fn expires_in(&self) -> Option<i32> { self.expires_in.clone() }
  #[doc(hidden)] pub fn _set_expires_in(&mut self, expires_in: i32) -> &mut Self { self.expires_in = Some(expires_in); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



