
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a Telegram Passport element that was requested by a service. 
#[derive(Debug, Serialize, Deserialize)]
pub struct PassportSuitableElement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // passportSuitableElement
  /// Type of the element.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<PassportElementType>>,
  /// True, if a selfie is required with the identity document.
  is_selfie_required: Option<bool>,
  /// True, if a certified English translation is required with the document.
  is_translation_required: Option<bool>,
  /// True, if personal details must include the user's name in the language of their country of residence.
  is_native_name_required: Option<bool>,
  
}


impl Clone for PassportSuitableElement {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for PassportSuitableElement {}
impl RObject for PassportSuitableElement {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "passportSuitableElement" }
  fn td_type(&self) -> RTDType { RTDType::PassportSuitableElement }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl PassportSuitableElement {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "passportSuitableElement".to_string(),
      type_: None,
      is_selfie_required: None,
      is_translation_required: None,
      is_native_name_required: None,
      
    }
  }
  
  pub fn type_(&self) -> Option<Box<PassportElementType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<PassportElementType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn is_selfie_required(&self) -> Option<bool> { self.is_selfie_required.clone() }
  #[doc(hidden)] pub fn _set_is_selfie_required(&mut self, is_selfie_required: bool) -> &mut Self { self.is_selfie_required = Some(is_selfie_required); self }
  
  pub fn is_translation_required(&self) -> Option<bool> { self.is_translation_required.clone() }
  #[doc(hidden)] pub fn _set_is_translation_required(&mut self, is_translation_required: bool) -> &mut Self { self.is_translation_required = Some(is_translation_required); self }
  
  pub fn is_native_name_required(&self) -> Option<bool> { self.is_native_name_required.clone() }
  #[doc(hidden)] pub fn _set_is_native_name_required(&mut self, is_native_name_required: bool) -> &mut Self { self.is_native_name_required = Some(is_native_name_required); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



