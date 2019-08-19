
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns background wallpapers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWallpapers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getWallpapers
  
}



impl Object for GetWallpapers {}
impl RObject for GetWallpapers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getWallpapers" }
  fn td_type(&self) -> RTDType { RTDType::GetWallpapers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetWallpapers {}


impl GetWallpapers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getWallpapers".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



