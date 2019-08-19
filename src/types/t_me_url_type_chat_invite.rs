
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat invite link. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMeUrlTypeChatInvite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // tMeUrlTypeChatInvite
  /// Chat invite link info.
  info: Option<ChatInviteLinkInfo>,
  
}



impl Object for TMeUrlTypeChatInvite {}
impl RObject for TMeUrlTypeChatInvite {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrlTypeChatInvite" }
  fn td_type(&self) -> RTDType { RTDType::TMeUrlTypeChatInvite }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl TMeUrlType for TMeUrlTypeChatInvite {}


impl TMeUrlTypeChatInvite {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "tMeUrlTypeChatInvite".to_string(),
      info: None,
      
    }
  }
  
  pub fn info(&self) -> Option<ChatInviteLinkInfo> { self.info.clone() }
  #[doc(hidden)] pub fn _set_info(&mut self, info: ChatInviteLinkInfo) -> &mut Self { self.info = Some(info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



