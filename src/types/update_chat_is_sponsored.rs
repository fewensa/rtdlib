
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A chat's is_sponsored field has changed. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatIsSponsored {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // updateChatIsSponsored
  /// Chat identifier.
  chat_id: Option<i64>,
  /// New value of is_sponsored.
  is_sponsored: Option<bool>,
  /// New value of chat order.
  order: Option<i64>,
  
}



impl Object for UpdateChatIsSponsored {}
impl RObject for UpdateChatIsSponsored {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "updateChatIsSponsored" }
  fn td_type(&self) -> RTDType { RTDType::UpdateChatIsSponsored }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl Update for UpdateChatIsSponsored {}


impl UpdateChatIsSponsored {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "updateChatIsSponsored".to_string(),
      chat_id: None,
      is_sponsored: None,
      order: None,
      
    }
  }
  
  pub fn chat_id(&self) -> Option<i64> { self.chat_id.clone() }
  #[doc(hidden)] pub fn _set_chat_id(&mut self, chat_id: i64) -> &mut Self { self.chat_id = Some(chat_id); self }
  
  pub fn is_sponsored(&self) -> Option<bool> { self.is_sponsored.clone() }
  #[doc(hidden)] pub fn _set_is_sponsored(&mut self, is_sponsored: bool) -> &mut Self { self.is_sponsored = Some(is_sponsored); self }
  
  pub fn order(&self) -> Option<i64> { self.order.clone() }
  #[doc(hidden)] pub fn _set_order(&mut self, order: i64) -> &mut Self { self.order = Some(order); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



