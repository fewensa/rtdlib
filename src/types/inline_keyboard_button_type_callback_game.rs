
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A button with a game that sends a special callback query to a bot. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeCallbackGame {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineKeyboardButtonTypeCallbackGame
  
}



impl Object for InlineKeyboardButtonTypeCallbackGame {}
impl RObject for InlineKeyboardButtonTypeCallbackGame {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeCallbackGame" }
  fn td_type(&self) -> RTDType { RTDType::InlineKeyboardButtonTypeCallbackGame }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineKeyboardButtonType for InlineKeyboardButtonTypeCallbackGame {}


impl InlineKeyboardButtonTypeCallbackGame {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineKeyboardButtonTypeCallbackGame".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



