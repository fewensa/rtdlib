
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of trending sticker sets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrendingStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getTrendingStickerSets
  
}



impl Object for GetTrendingStickerSets {}
impl RObject for GetTrendingStickerSets {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getTrendingStickerSets" }
  fn td_type(&self) -> RTDType { RTDType::GetTrendingStickerSets }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetTrendingStickerSets {}


impl GetTrendingStickerSets {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getTrendingStickerSets".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



