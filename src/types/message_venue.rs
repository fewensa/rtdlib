
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message with information about a venue. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageVenue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageVenue
  /// Message content.
  venue: Option<Venue>,
  
}



impl Object for MessageVenue {}
impl RObject for MessageVenue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageVenue" }
  fn td_type(&self) -> RTDType { RTDType::MessageVenue }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageVenue {}


impl MessageVenue {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageVenue".to_string(),
      venue: None,
      
    }
  }
  
  pub fn venue(&self) -> Option<Venue> { self.venue.clone() }
  #[doc(hidden)] pub fn _set_venue(&mut self, venue: Venue) -> &mut Self { self.venue = Some(venue); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



