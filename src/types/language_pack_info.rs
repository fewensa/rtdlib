
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a language pack. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguagePackInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // languagePackInfo
  /// Unique language pack identifier.
  id: Option<String>,
  /// Identifier of a base language pack; may be empty. If a string is missed in the language pack, then it should be fetched from base language pack. Unsupported in custom language packs.
  base_language_pack_id: Option<String>,
  /// Language name.
  name: Option<String>,
  /// Name of the language in that language.
  native_name: Option<String>,
  /// A language code to be used to apply plural forms. See https://www.unicode.org/cldr/charts/latest/supplemental/language_plural_rules.html for more info.
  plural_code: Option<String>,
  /// True, if the language pack is official.
  is_official: Option<bool>,
  /// True, if the language pack strings are RTL.
  is_rtl: Option<bool>,
  /// True, if the language pack is a beta language pack.
  is_beta: Option<bool>,
  /// True, if the language pack is installed by the current user.
  is_installed: Option<bool>,
  /// Total number of non-deleted strings from the language pack.
  total_string_count: Option<i32>,
  /// Total number of translated strings from the language pack.
  translated_string_count: Option<i32>,
  /// Total number of non-deleted strings from the language pack available locally.
  local_string_count: Option<i32>,
  /// Link to language translation interface; empty for custom local language packs.
  translation_url: Option<String>,
  
}



impl Object for LanguagePackInfo {}
impl RObject for LanguagePackInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "languagePackInfo" }
  fn td_type(&self) -> RTDType { RTDType::LanguagePackInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl LanguagePackInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "languagePackInfo".to_string(),
      id: None,
      base_language_pack_id: None,
      name: None,
      native_name: None,
      plural_code: None,
      is_official: None,
      is_rtl: None,
      is_beta: None,
      is_installed: None,
      total_string_count: None,
      translated_string_count: None,
      local_string_count: None,
      translation_url: None,
      
    }
  }
  
  pub fn id(&self) -> Option<String> { self.id.clone() }
  #[doc(hidden)] pub fn _set_id(&mut self, id: String) -> &mut Self { self.id = Some(id); self }
  
  pub fn base_language_pack_id(&self) -> Option<String> { self.base_language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_base_language_pack_id(&mut self, base_language_pack_id: String) -> &mut Self { self.base_language_pack_id = Some(base_language_pack_id); self }
  
  pub fn name(&self) -> Option<String> { self.name.clone() }
  #[doc(hidden)] pub fn _set_name(&mut self, name: String) -> &mut Self { self.name = Some(name); self }
  
  pub fn native_name(&self) -> Option<String> { self.native_name.clone() }
  #[doc(hidden)] pub fn _set_native_name(&mut self, native_name: String) -> &mut Self { self.native_name = Some(native_name); self }
  
  pub fn plural_code(&self) -> Option<String> { self.plural_code.clone() }
  #[doc(hidden)] pub fn _set_plural_code(&mut self, plural_code: String) -> &mut Self { self.plural_code = Some(plural_code); self }
  
  pub fn is_official(&self) -> Option<bool> { self.is_official.clone() }
  #[doc(hidden)] pub fn _set_is_official(&mut self, is_official: bool) -> &mut Self { self.is_official = Some(is_official); self }
  
  pub fn is_rtl(&self) -> Option<bool> { self.is_rtl.clone() }
  #[doc(hidden)] pub fn _set_is_rtl(&mut self, is_rtl: bool) -> &mut Self { self.is_rtl = Some(is_rtl); self }
  
  pub fn is_beta(&self) -> Option<bool> { self.is_beta.clone() }
  #[doc(hidden)] pub fn _set_is_beta(&mut self, is_beta: bool) -> &mut Self { self.is_beta = Some(is_beta); self }
  
  pub fn is_installed(&self) -> Option<bool> { self.is_installed.clone() }
  #[doc(hidden)] pub fn _set_is_installed(&mut self, is_installed: bool) -> &mut Self { self.is_installed = Some(is_installed); self }
  
  pub fn total_string_count(&self) -> Option<i32> { self.total_string_count.clone() }
  #[doc(hidden)] pub fn _set_total_string_count(&mut self, total_string_count: i32) -> &mut Self { self.total_string_count = Some(total_string_count); self }
  
  pub fn translated_string_count(&self) -> Option<i32> { self.translated_string_count.clone() }
  #[doc(hidden)] pub fn _set_translated_string_count(&mut self, translated_string_count: i32) -> &mut Self { self.translated_string_count = Some(translated_string_count); self }
  
  pub fn local_string_count(&self) -> Option<i32> { self.local_string_count.clone() }
  #[doc(hidden)] pub fn _set_local_string_count(&mut self, local_string_count: i32) -> &mut Self { self.local_string_count = Some(local_string_count); self }
  
  pub fn translation_url(&self) -> Option<String> { self.translation_url.clone() }
  #[doc(hidden)] pub fn _set_translation_url(&mut self, translation_url: String) -> &mut Self { self.translation_url = Some(translation_url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



