
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a venue. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // venue
  /// Venue location; as defined by the sender.
  location: Option<Location>,
  /// Venue name; as defined by the sender.
  title: Option<String>,
  /// Venue address; as defined by the sender.
  address: Option<String>,
  /// Provider of the venue database; as defined by the sender. Currently only "foursquare" needs to be supported.
  provider: Option<String>,
  /// Identifier of the venue in the provider database; as defined by the sender.
  id: Option<String>,
  /// Type of the venue in the provider database; as defined by the sender.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<String>,
  
}



impl Object for Venue {}
impl RObject for Venue {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "venue" }
  fn td_type(&self) -> RTDType { RTDType::Venue }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Venue {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "venue".to_string(),
      location: None,
      title: None,
      address: None,
      provider: None,
      id: None,
      type_: None,
      
    }
  }
  
  pub fn location(&self) -> Option<Location> { self.location.clone() }
  #[doc(hidden)] pub fn _set_location(&mut self, location: Location) -> &mut Self { self.location = Some(location); self }
  
  pub fn title(&self) -> Option<String> { self.title.clone() }
  #[doc(hidden)] pub fn _set_title(&mut self, title: String) -> &mut Self { self.title = Some(title); self }
  
  pub fn address(&self) -> Option<String> { self.address.clone() }
  #[doc(hidden)] pub fn _set_address(&mut self, address: String) -> &mut Self { self.address = Some(address); self }
  
  pub fn provider(&self) -> Option<String> { self.provider.clone() }
  #[doc(hidden)] pub fn _set_provider(&mut self, provider: String) -> &mut Self { self.provider = Some(provider); self }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn type_(&self) -> Option<String> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: String) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



