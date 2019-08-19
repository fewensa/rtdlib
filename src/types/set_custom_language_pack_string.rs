
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds, edits or deletes a string in a custom local language pack. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetCustomLanguagePackString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // setCustomLanguagePackString
  /// Identifier of a previously added custom local language pack in the current localization target.
  language_pack_id: Option<String>,
  /// New language pack string.
  new_string: Option<LanguagePackString>,
  
}



impl Object for SetCustomLanguagePackString {}
impl RObject for SetCustomLanguagePackString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "setCustomLanguagePackString" }
  fn td_type(&self) -> RTDType { RTDType::SetCustomLanguagePackString }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SetCustomLanguagePackString {}


impl SetCustomLanguagePackString {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "setCustomLanguagePackString".to_string(),
      language_pack_id: None,
      new_string: None,
      
    }
  }
  
  pub fn language_pack_id(&self) -> Option<String> { self.language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_language_pack_id(&mut self, language_pack_id: String) -> &mut Self { self.language_pack_id = Some(language_pack_id); self }
  
  pub fn new_string(&self) -> Option<LanguagePackString> { self.new_string.clone() }
  #[doc(hidden)] pub fn _set_new_string(&mut self, new_string: LanguagePackString) -> &mut Self { self.new_string = Some(new_string); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



