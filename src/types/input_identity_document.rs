
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An identity document to be saved to Telegram Passport. 
#[derive(Debug, Serialize, Deserialize)]
pub struct InputIdentityDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // inputIdentityDocument
  /// Document number; 1-24 characters.
  number: Option<String>,
  /// Document expiry date, if available.
  expiry_date: Option<Date>,
  /// Front side of the document.
  front_side: Option<Box<InputFile>>,
  /// Reverse side of the document; only for driver license and identity card.
  reverse_side: Option<Box<InputFile>>,
  /// Selfie with the document, if available.
  selfie: Option<Box<InputFile>>,
  /// List of files containing a certified English translation of the document.
  translation: Option<Vec<Box<InputFile>>>,
  
}


impl Clone for InputIdentityDocument {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for InputIdentityDocument {}
impl RObject for InputIdentityDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "inputIdentityDocument" }
  fn td_type(&self) -> RTDType { RTDType::InputIdentityDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl InputIdentityDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "inputIdentityDocument".to_string(),
      number: None,
      expiry_date: None,
      front_side: None,
      reverse_side: None,
      selfie: None,
      translation: None,
      
    }
  }
  
  pub fn number(&self) -> Option<String> { self.number.clone() }
  #[doc(hidden)] pub fn _set_number(&mut self, number: String) -> &mut Self { self.number = Some(number); self }
  
  pub fn expiry_date(&self) -> Option<Date> { self.expiry_date.clone() }
  #[doc(hidden)] pub fn _set_expiry_date(&mut self, expiry_date: Date) -> &mut Self { self.expiry_date = Some(expiry_date); self }
  
  pub fn front_side(&self) -> Option<Box<InputFile>> { self.front_side.clone() }
  #[doc(hidden)] pub fn _set_front_side(&mut self, front_side: Box<InputFile>) -> &mut Self { self.front_side = Some(front_side); self }
  
  pub fn reverse_side(&self) -> Option<Box<InputFile>> { self.reverse_side.clone() }
  #[doc(hidden)] pub fn _set_reverse_side(&mut self, reverse_side: Box<InputFile>) -> &mut Self { self.reverse_side = Some(reverse_side); self }
  
  pub fn selfie(&self) -> Option<Box<InputFile>> { self.selfie.clone() }
  #[doc(hidden)] pub fn _set_selfie(&mut self, selfie: Box<InputFile>) -> &mut Self { self.selfie = Some(selfie); self }
  
  pub fn translation(&self) -> Option<Vec<Box<InputFile>>> { self.translation.clone() }
  #[doc(hidden)] pub fn _set_translation(&mut self, translation: Vec<Box<InputFile>>) -> &mut Self { self.translation = Some(translation); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



