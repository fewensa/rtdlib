
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes a location on planet Earth. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // location
  /// Latitude of the location in degrees; as defined by the sender.
  latitude: Option<f64>,
  /// Longitude of the location, in degrees; as defined by the sender.
  longitude: Option<f64>,
  
}



impl Object for Location {}
impl RObject for Location {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "location" }
  fn td_type(&self) -> RTDType { RTDType::Location }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Location {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "location".to_string(),
      latitude: None,
      longitude: None,
      
    }
  }
  
  pub fn latitude(&self) -> Option<f64> { self.latitude.clone() }
  #[doc(hidden)] pub fn _set_latitude(&mut self, latitude: f64) -> &mut Self { self.latitude = Some(latitude); self }
  
  pub fn longitude(&self) -> Option<f64> { self.longitude.clone() }
  #[doc(hidden)] pub fn _set_longitude(&mut self, longitude: f64) -> &mut Self { self.longitude = Some(longitude); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



