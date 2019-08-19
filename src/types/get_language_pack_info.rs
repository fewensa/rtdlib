
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a language pack. Returned language pack identifier may be different from a provided one. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLanguagePackInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getLanguagePackInfo
  /// Language pack identifier.
  language_pack_id: Option<String>,
  
}



impl Object for GetLanguagePackInfo {}
impl RObject for GetLanguagePackInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getLanguagePackInfo" }
  fn td_type(&self) -> RTDType { RTDType::GetLanguagePackInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetLanguagePackInfo {}


impl GetLanguagePackInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getLanguagePackInfo".to_string(),
      language_pack_id: None,
      
    }
  }
  
  pub fn language_pack_id(&self) -> Option<String> { self.language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_language_pack_id(&mut self, language_pack_id: String) -> &mut Self { self.language_pack_id = Some(language_pack_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



