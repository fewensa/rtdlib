
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rich text URL link. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextUrl
  /// Text.
  text: Option<Box<RichText>>,
  /// URL.
  url: Option<String>,
  
}


impl Clone for RichTextUrl {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextUrl {}
impl RObject for RichTextUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextUrl" }
  fn td_type(&self) -> RTDType { RTDType::RichTextUrl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextUrl {}


impl RichTextUrl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextUrl".to_string(),
      text: None,
      url: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



