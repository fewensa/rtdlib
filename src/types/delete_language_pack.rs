
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Deletes all information about a language pack in the current localization target. The language pack which is currently in use (including base language pack) or is being synchronized can't be deleted. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deleteLanguagePack
  /// Identifier of the language pack to delete.
  language_pack_id: Option<String>,
  
}



impl Object for DeleteLanguagePack {}
impl RObject for DeleteLanguagePack {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deleteLanguagePack" }
  fn td_type(&self) -> RTDType { RTDType::DeleteLanguagePack }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for DeleteLanguagePack {}


impl DeleteLanguagePack {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deleteLanguagePack".to_string(),
      language_pack_id: None,
      
    }
  }
  
  pub fn language_pack_id(&self) -> Option<String> { self.language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_language_pack_id(&mut self, language_pack_id: String) -> &mut Self { self.language_pack_id = Some(language_pack_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



