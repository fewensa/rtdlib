
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns list of available TDLib internal log tags, for example, ["actor", "binlog", "connections", "notifications", "proxy"]. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLogTags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getLogTags
  
}



impl Object for GetLogTags {}
impl RObject for GetLogTags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLogTags" }
  fn td_type(&self) -> RTDType { RTDType::GetLogTags }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetLogTags {}


impl GetLogTags {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getLogTags".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



