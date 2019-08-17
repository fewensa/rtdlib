
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Uses current user IP to found his country. Returns two-letter ISO 3166-1 alpha-2 country code. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCountryCode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getCountryCode
  
}



impl Object for GetCountryCode {}
impl RObject for GetCountryCode {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getCountryCode" }
  fn td_type(&self) -> RTDType { RTDType::GetCountryCode }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetCountryCode {}


impl GetCountryCode {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getCountryCode".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



