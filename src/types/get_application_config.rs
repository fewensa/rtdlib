
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns application config, provided by the server. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationConfig {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getApplicationConfig
  
}



impl Object for GetApplicationConfig {}
impl RObject for GetApplicationConfig {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getApplicationConfig" }
  fn td_type(&self) -> RTDType { RTDType::GetApplicationConfig }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetApplicationConfig {}


impl GetApplicationConfig {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getApplicationConfig".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



