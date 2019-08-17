
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns up to 20 recently used inline bots in the order of their last usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRecentInlineBots {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getRecentInlineBots
  
}



impl Object for GetRecentInlineBots {}
impl RObject for GetRecentInlineBots {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getRecentInlineBots" }
  fn td_type(&self) -> RTDType { RTDType::GetRecentInlineBots }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetRecentInlineBots {}


impl GetRecentInlineBots {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getRecentInlineBots".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



