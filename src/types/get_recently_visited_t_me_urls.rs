
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns t.me URLs recently visited by a newly registered user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentlyVisitedTMeUrls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getRecentlyVisitedTMeUrls
  /// Google Play referrer to identify the user.
  referrer: Option<String>,
  
}



impl Object for GetRecentlyVisitedTMeUrls {}
impl RObject for GetRecentlyVisitedTMeUrls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRecentlyVisitedTMeUrls" }
  fn td_type(&self) -> RTDType { RTDType::GetRecentlyVisitedTMeUrls }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetRecentlyVisitedTMeUrls {}


impl GetRecentlyVisitedTMeUrls {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getRecentlyVisitedTMeUrls".to_string(),
      referrer: None,
      
    }
  }
  
  pub fn referrer(&self) -> Option<String> { self.referrer.clone() }
  #[doc(hidden)] pub fn _set_referrer(&mut self, referrer: String) -> &mut Self { self.referrer = Some(referrer); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



