
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns network data usage statistics. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetNetworkStatistics {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getNetworkStatistics
  /// If true, returns only data for the current library launch.
  only_current: Option<bool>,
  
}



impl Object for GetNetworkStatistics {}
impl RObject for GetNetworkStatistics {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getNetworkStatistics" }
  fn td_type(&self) -> RTDType { RTDType::GetNetworkStatistics }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetNetworkStatistics {}


impl GetNetworkStatistics {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getNetworkStatistics".to_string(),
      only_current: None,
      
    }
  }
  
  pub fn only_current(&self) -> Option<bool> { self.only_current.clone() }
  #[doc(hidden)] pub fn _set_only_current(&mut self, only_current: bool) -> &mut Self { self.only_current = Some(only_current); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



