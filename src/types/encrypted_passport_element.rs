
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about an encrypted Telegram Passport element; for bots only. 
#[derive(Debug, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // encryptedPassportElement
  /// Type of Telegram Passport element.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<PassportElementType>>,
  /// Encrypted JSON-encoded data about the user.
  data: Option<String>,
  /// The front side of an identity document.
  front_side: Option<DatedFile>,
  /// The reverse side of an identity document; may be null.
  reverse_side: Option<DatedFile>,
  /// Selfie with the document; may be null.
  selfie: Option<DatedFile>,
  /// List of files containing a certified English translation of the document.
  translation: Option<Vec<DatedFile>>,
  /// List of attached files.
  files: Option<Vec<DatedFile>>,
  /// Unencrypted data, phone number or email address.
  value: Option<String>,
  /// Hash of the entire element.
  hash: Option<String>,
  
}


impl Clone for EncryptedPassportElement {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for EncryptedPassportElement {}
impl RObject for EncryptedPassportElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "encryptedPassportElement" }
  fn td_type(&self) -> RTDType { RTDType::EncryptedPassportElement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl EncryptedPassportElement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "encryptedPassportElement".to_string(),
      type_: None,
      data: None,
      front_side: None,
      reverse_side: None,
      selfie: None,
      translation: None,
      files: None,
      value: None,
      hash: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<Box<PassportElementType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<PassportElementType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn data(&self) -> Option<String> { self.data.clone() }
  #[doc(hidden)] pub fn _set_data(&mut self, data: String) -> &mut Self { self.data = Some(data); self }
  
  pub fn front_side(&self) -> Option<DatedFile> { self.front_side.clone() }
  #[doc(hidden)] pub fn _set_front_side(&mut self, front_side: DatedFile) -> &mut Self { self.front_side = Some(front_side); self }
  
  pub fn reverse_side(&self) -> Option<DatedFile> { self.reverse_side.clone() }
  #[doc(hidden)] pub fn _set_reverse_side(&mut self, reverse_side: DatedFile) -> &mut Self { self.reverse_side = Some(reverse_side); self }
  
  pub fn selfie(&self) -> Option<DatedFile> { self.selfie.clone() }
  #[doc(hidden)] pub fn _set_selfie(&mut self, selfie: DatedFile) -> &mut Self { self.selfie = Some(selfie); self }
  
  pub fn translation(&self) -> Option<Vec<DatedFile>> { self.translation.clone() }
  #[doc(hidden)] pub fn _set_translation(&mut self, translation: Vec<DatedFile>) -> &mut Self { self.translation = Some(translation); self }
  
  pub fn files(&self) -> Option<Vec<DatedFile>> { self.files.clone() }
  #[doc(hidden)] pub fn _set_files(&mut self, files: Vec<DatedFile>) -> &mut Self { self.files = Some(files); self }
  
  pub fn value(&self) -> Option<String> { self.value.clone() }
  #[doc(hidden)] pub fn _set_value(&mut self, value: String) -> &mut Self { self.value = Some(value); self }
  
  pub fn hash(&self) -> Option<String> { self.hash.clone() }
  #[doc(hidden)] pub fn _set_hash(&mut self, hash: String) -> &mut Self { self.hash = Some(hash); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



