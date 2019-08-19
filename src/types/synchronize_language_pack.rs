
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Fetches the latest versions of all strings from a language pack in the current localization target from the server. This method doesn't need to be called explicitly for the current used/base language packs. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SynchronizeLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // synchronizeLanguagePack
  /// Language pack identifier.
  language_pack_id: Option<String>,
  
}



impl Object for SynchronizeLanguagePack {}
impl RObject for SynchronizeLanguagePack {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "synchronizeLanguagePack" }
  fn td_type(&self) -> RTDType { RTDType::SynchronizeLanguagePack }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SynchronizeLanguagePack {}


impl SynchronizeLanguagePack {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "synchronizeLanguagePack".to_string(),
      language_pack_id: None,
      
    }
  }
  
  pub fn language_pack_id(&self) -> Option<String> { self.language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_language_pack_id(&mut self, language_pack_id: String) -> &mut Self { self.language_pack_id = Some(language_pack_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



