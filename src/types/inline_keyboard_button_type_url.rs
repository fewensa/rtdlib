
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A button that opens a specified URL. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButtonTypeUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inlineKeyboardButtonTypeUrl
  /// HTTP or tg:// URL to open.
  url: Option<String>,
  
}



impl Object for InlineKeyboardButtonTypeUrl {}
impl RObject for InlineKeyboardButtonTypeUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inlineKeyboardButtonTypeUrl" }
  fn td_type(&self) -> RTDType { RTDType::InlineKeyboardButtonTypeUrl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl InlineKeyboardButtonType for InlineKeyboardButtonTypeUrl {}


impl InlineKeyboardButtonTypeUrl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inlineKeyboardButtonTypeUrl".to_string(),
      url: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



