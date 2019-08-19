
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with a location. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessageLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputMessageLocation
  /// Location to be sent.
  location: Option<Location>,
  /// Period for which the location can be updated, in seconds; should bebetween 60 and 86400 for a live location and 0 otherwise.
  live_period: Option<i32>,
  
}



impl Object for InputMessageLocation {}
impl RObject for InputMessageLocation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputMessageLocation" }
  fn td_type(&self) -> RTDType { RTDType::InputMessageLocation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InputMessageContent for InputMessageLocation {}


impl InputMessageLocation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputMessageLocation".to_string(),
      location: None,
      live_period: None,
      
    }
  }
  
  pub fn location(&self) -> Option<Location> { self.location.clone() }
  #[doc(hidden)] pub fn _set_location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  
  pub fn live_period(&self) -> Option<i32> { self.live_period.clone() }
  #[doc(hidden)] pub fn _set_live_period(&mut self, live_period: i32) -> &mut Self { self.live_period = Some(live_period); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



