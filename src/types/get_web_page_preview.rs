
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a web page preview by the text of the message. Do not call this function too often. Returns a 404 error if the web page has no preview.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebPagePreview {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getWebPagePreview
  /// Message text with formatting.
  text: Option<FormattedText>,
  
}



impl Object for GetWebPagePreview {}
impl RObject for GetWebPagePreview {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getWebPagePreview" }
  fn td_type(&self) -> RTDType { RTDType::GetWebPagePreview }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetWebPagePreview {}


impl GetWebPagePreview {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getWebPagePreview".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<FormattedText> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: FormattedText) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



