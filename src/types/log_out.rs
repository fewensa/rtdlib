
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Closes the TDLib instance after a proper logout. Requires an available network connection. All local data will be destroyed. After the logout completes, 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogOut {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // logOut
  
}



impl Object for LogOut {}
impl RObject for LogOut {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logOut" }
  fn td_type(&self) -> RTDType { RTDType::LogOut }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for LogOut {}


impl LogOut {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "logOut".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



