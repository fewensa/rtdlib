
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a list of public chats created by the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCreatedPublicChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getCreatedPublicChats
  
}



impl Object for GetCreatedPublicChats {}
impl RObject for GetCreatedPublicChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getCreatedPublicChats" }
  fn td_type(&self) -> RTDType { RTDType::GetCreatedPublicChats }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetCreatedPublicChats {}


impl GetCreatedPublicChats {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getCreatedPublicChats".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



