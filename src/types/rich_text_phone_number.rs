
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A rich text phone number. 
#[derive(Debug, Serialize, Deserialize)]
pub struct RichTextPhoneNumber {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // richTextPhoneNumber
  /// Text.
  text: Option<Box<RichText>>,
  /// Phone number.
  phone_number: Option<String>,
  
}


impl Clone for RichTextPhoneNumber {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for RichTextPhoneNumber {}
impl RObject for RichTextPhoneNumber {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "richTextPhoneNumber" }
  fn td_type(&self) -> RTDType { RTDType::RichTextPhoneNumber }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl RichText for RichTextPhoneNumber {}


impl RichTextPhoneNumber {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "richTextPhoneNumber".to_string(),
      text: None,
      phone_number: None,
      
    }
  }
  
  pub fn text(&self) -> Option<Box<RichText>> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: Box<RichText>) -> &mut Self { self.text = Some(text); self }
  
  pub fn phone_number(&self) -> Option<String> { self.phone_number.clone() }
  #[doc(hidden)] pub fn _set_phone_number(&mut self, phone_number: String) -> &mut Self { self.phone_number = Some(phone_number); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



