
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A button that sends a special callback query to a bot. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallback {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineKeyboardButtonTypeCallback
  /// Data to be sent to the bot via a callback query.
  data: Option<String>,
  
}



impl Object for InlineKeyboardButtonTypeCallback {}
impl RObject for InlineKeyboardButtonTypeCallback {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeCallback" }
  fn td_type(&self) -> RTDType { RTDType::InlineKeyboardButtonTypeCallback }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineKeyboardButtonType for InlineKeyboardButtonTypeCallback {}


impl InlineKeyboardButtonTypeCallback {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineKeyboardButtonTypeCallback".to_string(),
      data: None,
      
    }
  }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



