
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Instructs clients to force a reply to this message. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyMarkupForceReply {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // replyMarkupForceReply
  /// True, if a forced reply must automatically be shown to the current user. For outgoing messages, specify true to show the forced reply only for the mentioned users and for the target user of a reply.
  is_personal: Option<bool>,
  
}



impl Object for ReplyMarkupForceReply {}
impl RObject for ReplyMarkupForceReply {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "replyMarkupForceReply" }
  fn td_type(&self) -> RTDType { RTDType::ReplyMarkupForceReply }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ReplyMarkup for ReplyMarkupForceReply {}


impl ReplyMarkupForceReply {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "replyMarkupForceReply".to_string(),
      is_personal: None,
      
    }
  }
  
  pub fn is_personal(&self) -> Option<bool> { self.is_personal.clone() }
  #[doc(hidden)] pub fn _set_is_personal(&mut self, is_personal: bool) -> &mut Self { self.is_personal = Some(is_personal); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



