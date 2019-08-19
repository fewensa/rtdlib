
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// An identity document. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // identityDocument
  /// Document number; 1-24 characters.
  number: Option<String>,
  /// Document expiry date; may be null.
  expiry_date: Option<Date>,
  /// Front side of the document.
  front_side: Option<DatedFile>,
  /// Reverse side of the document; only for driver license and identity card.
  reverse_side: Option<DatedFile>,
  /// Selfie with the document; may be null.
  selfie: Option<DatedFile>,
  /// List of files containing a certified English translation of the document.
  translation: Option<Vec<DatedFile>>,
  
}



impl Object for IdentityDocument {}
impl RObject for IdentityDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "identityDocument" }
  fn td_type(&self) -> RTDType { RTDType::IdentityDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl IdentityDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "identityDocument".to_string(),
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
  
  pub fn front_side(&self) -> Option<DatedFile> { self.front_side.clone() }
  #[doc(hidden)] pub fn _set_front_side(&mut self, front_side: DatedFile) -> &mut Self { self.front_side = Some(front_side); self }
  
  pub fn reverse_side(&self) -> Option<DatedFile> { self.reverse_side.clone() }
  #[doc(hidden)] pub fn _set_reverse_side(&mut self, reverse_side: DatedFile) -> &mut Self { self.reverse_side = Some(reverse_side); self }
  
  pub fn selfie(&self) -> Option<DatedFile> { self.selfie.clone() }
  #[doc(hidden)] pub fn _set_selfie(&mut self, selfie: DatedFile) -> &mut Self { self.selfie = Some(selfie); self }
  
  pub fn translation(&self) -> Option<Vec<DatedFile>> { self.translation.clone() }
  #[doc(hidden)] pub fn _set_translation(&mut self, translation: Vec<DatedFile>) -> &mut Self { self.translation = Some(translation); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



