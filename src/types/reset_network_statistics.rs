
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Resets all network data usage statistics to zero. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetNetworkStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // resetNetworkStatistics
  
}



impl Object for ResetNetworkStatistics {}
impl RObject for ResetNetworkStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "resetNetworkStatistics" }
  fn td_type(&self) -> RTDType { RTDType::ResetNetworkStatistics }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ResetNetworkStatistics {}


impl ResetNetworkStatistics {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "resetNetworkStatistics".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



