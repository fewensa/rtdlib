
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns list of proxies that are currently set up. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProxies {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getProxies
  
}



impl Object for GetProxies {}
impl RObject for GetProxies {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getProxies" }
  fn td_type(&self) -> RTDType { RTDType::GetProxies }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetProxies {}


impl GetProxies {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getProxies".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



