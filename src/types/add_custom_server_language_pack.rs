
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Adds a custom server language pack to the list of installed language packs in current localization target. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddCustomServerLanguagePack {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // addCustomServerLanguagePack
  /// Identifier of a language pack to be added; may be different from a name that is used in an "https://t.me/setlanguage/" link.
  language_pack_id: Option<String>,
  
}



impl Object for AddCustomServerLanguagePack {}
impl RObject for AddCustomServerLanguagePack {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "addCustomServerLanguagePack" }
  fn td_type(&self) -> RTDType { RTDType::AddCustomServerLanguagePack }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for AddCustomServerLanguagePack {}


impl AddCustomServerLanguagePack {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "addCustomServerLanguagePack".to_string(),
      language_pack_id: None,
      
    }
  }
  
  pub fn language_pack_id(&self) -> Option<String> { self.language_pack_id.clone() }
  #[doc(hidden)] pub fn _set_language_pack_id(&mut self, language_pack_id: String) -> &mut Self { self.language_pack_id = Some(language_pack_id); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



