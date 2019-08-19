
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rich text email link. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextEmailAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextEmailAddress
  /// Text.
  text: Option<Box<RichText>>,
  /// Email address.
  email_address: Option<String>,
  
}


impl Clone for RichTextEmailAddress {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextEmailAddress {}
impl RObject for RichTextEmailAddress {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextEmailAddress" }
  fn td_type(&self) -> RTDType { RTDType::RichTextEmailAddress }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextEmailAddress {}


impl RichTextEmailAddress {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextEmailAddress".to_string(),
      text: None,
      email_address: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn email_address(&self) -> Option<String> { self.email_address.clone() }
  #[doc(hidden)] pub fn _set_email_address(&mut self, email_address: String) -> &mut Self { self.email_address = Some(email_address); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



