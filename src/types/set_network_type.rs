
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Sets the current network type. Can be called before authorization. Calling this method forces all network connections to reopen, mitigating the delay in switching between different networks, so it should be called whenever the network is changed, even if the network type remains the same. Network type is used to check whether the library can use the network at all and also for collecting detailed network data usage statistics.
#[derive(Debug, Serialize, Deserialize)]
pub struct SetNetworkType {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setNetworkType
  /// The new network type. By default, networkTypeOther.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<NetworkType>>,
  
}


impl Clone for SetNetworkType {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for SetNetworkType {}
impl RObject for SetNetworkType {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setNetworkType" }
  fn td_type(&self) -> RTDType { RTDType::SetNetworkType }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetNetworkType {}


impl SetNetworkType {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setNetworkType".to_string(),
      type_: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<Box<NetworkType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<NetworkType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



