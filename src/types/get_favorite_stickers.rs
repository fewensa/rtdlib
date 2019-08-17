
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns favorite stickers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFavoriteStickers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getFavoriteStickers
  
}



impl Object for GetFavoriteStickers {}
impl RObject for GetFavoriteStickers {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getFavoriteStickers" }
  fn td_type(&self) -> RTDType { RTDType::GetFavoriteStickers }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetFavoriteStickers {}


impl GetFavoriteStickers {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getFavoriteStickers".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



