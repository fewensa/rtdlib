
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Clears the list of recently found chats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClearRecentlyFoundChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // clearRecentlyFoundChats
  
}



impl Object for ClearRecentlyFoundChats {}
impl RObject for ClearRecentlyFoundChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "clearRecentlyFoundChats" }
  fn td_type(&self) -> RTDType { RTDType::ClearRecentlyFoundChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for ClearRecentlyFoundChats {}


impl ClearRecentlyFoundChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "clearRecentlyFoundChats".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



