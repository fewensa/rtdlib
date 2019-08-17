
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns strings from a language pack in the current localization target by their keys. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLanguagePackStrings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getLanguagePackStrings
  /// Language pack identifier of the strings to be returned.
  language_pack_id: Option<String>,
  /// Language pack keys of the strings to be returned; leave empty to request all available strings.
  keys: Option<Vec<String>>,
  
}



impl Object for GetLanguagePackStrings {}
impl RObject for GetLanguagePackStrings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLanguagePackStrings" }
  fn td_type(&self) -> RTDType { RTDType::GetLanguagePackStrings }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetLanguagePackStrings {}


impl GetLanguagePackStrings {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getLanguagePackStrings".to_string(),
      language_pack_id: None,
      keys: None,
      
    }
  }
  
  pub fn language_pack_id(&self) -> Option<String> { self.language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_language_pack_id(&mut self, language_pack_id: String) -> &mut Self { self.language_pack_id = Some(language_pack_id); self }
  
  pub fn keys(&self) -> Option<Vec<String>> { self.keys.clone() }
  #[doc(hidden)] pub fn _set_keys(&mut self, keys: Vec<String>) -> &mut Self { self.keys = Some(keys); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



