
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Edits information about a custom local language pack in the current localization target. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditCustomLanguagePackInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // editCustomLanguagePackInfo
  /// New information about the custom local language pack.
  info: Option<LanguagePackInfo>,
  
}



impl Object for EditCustomLanguagePackInfo {}
impl RObject for EditCustomLanguagePackInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "editCustomLanguagePackInfo" }
  fn td_type(&self) -> RTDType { RTDType::EditCustomLanguagePackInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for EditCustomLanguagePackInfo {}


impl EditCustomLanguagePackInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "editCustomLanguagePackInfo".to_string(),
      info: None,
      
    }
  }
  
  pub fn info(&self) -> Option<LanguagePackInfo> { self.info.clone() }
  #[doc(hidden)] pub fn _set_info(&mut self, info: LanguagePackInfo) -> &mut Self { self.info = Some(info); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



