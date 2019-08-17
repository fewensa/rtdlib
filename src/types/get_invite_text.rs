
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns the default text for invitation messages to be used as a placeholder when the current user invites friends to Telegram.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInviteText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getInviteText
  
}



impl Object for GetInviteText {}
impl RObject for GetInviteText {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getInviteText" }
  fn td_type(&self) -> RTDType { RTDType::GetInviteText }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetInviteText {}


impl GetInviteText {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getInviteText".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



