
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A non-standard action has happened in the chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageCustomServiceAction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // messageCustomServiceAction
  /// Message text to be shown in the chat.
  text: Option<String>,
  
}



impl Object for MessageCustomServiceAction {}
impl RObject for MessageCustomServiceAction {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "messageCustomServiceAction" }
  fn td_type(&self) -> RTDType { RTDType::MessageCustomServiceAction }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl MessageContent for MessageCustomServiceAction {}


impl MessageCustomServiceAction {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "messageCustomServiceAction".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



