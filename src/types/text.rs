
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains some text. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Text {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // text
  /// Text.
  text: Option<String>,
  
}



impl Object for Text {}
impl RObject for Text {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "text" }
  fn td_type(&self) -> RTDType { RTDType::Text }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Text {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "text".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



