
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A button to buy something. This button must be in the first column and row of the keyboard and can be attached only to a message with content of the type 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeBuy {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineKeyboardButtonTypeBuy
  
}



impl Object for InlineKeyboardButtonTypeBuy {}
impl RObject for InlineKeyboardButtonTypeBuy {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeBuy" }
  fn td_type(&self) -> RTDType { RTDType::InlineKeyboardButtonTypeBuy }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineKeyboardButtonType for InlineKeyboardButtonTypeBuy {}


impl InlineKeyboardButtonTypeBuy {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineKeyboardButtonTypeBuy".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



