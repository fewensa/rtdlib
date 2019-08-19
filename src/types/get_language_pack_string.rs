
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns a string stored in the local database from the specified localization target and language pack by its key. Returns a 404 error if the string is not found. This is an offline method. Can be called before authorization. Can be called synchronously.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLanguagePackString {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getLanguagePackString
  /// Path to the language pack database in which strings are stored.
  language_pack_database_path: Option<String>,
  /// Localization target to which the language pack belongs.
  localization_target: Option<String>,
  /// Language pack identifier.
  language_pack_id: Option<String>,
  /// Language pack key of the string to be returned.
  key: Option<String>,
  
}



impl Object for GetLanguagePackString {}
impl RObject for GetLanguagePackString {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLanguagePackString" }
  fn td_type(&self) -> RTDType { RTDType::GetLanguagePackString }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetLanguagePackString {}


impl GetLanguagePackString {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getLanguagePackString".to_string(),
      language_pack_database_path: None,
      localization_target: None,
      language_pack_id: None,
      key: None,
      
    }
  }
  
  pub fn language_pack_database_path(&self) -> Option<String> { self.language_pack_database_path.clone() }
  #[doc(hidden)] pub fn _set_language_pack_database_path(&mut self, language_pack_database_path: String) -> &mut Self { self.language_pack_database_path = Some(language_pack_database_path); self }
  
  pub fn localization_target(&self) -> Option<String> { self.localization_target.clone() }
  #[doc(hidden)] pub fn _set_localization_target(&mut self, localization_target: String) -> &mut Self { self.localization_target = Some(localization_target); self }
  
  pub fn language_pack_id(&self) -> Option<String> { self.language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_language_pack_id(&mut self, language_pack_id: String) -> &mut Self { self.language_pack_id = Some(language_pack_id); self }
  
  pub fn key(&self) -> Option<String> { self.key.clone() }
  #[doc(hidden)] pub fn _set_key(&mut self, key: String) -> &mut Self { self.key = Some(key); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



