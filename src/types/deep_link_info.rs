
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about a tg:// deep link. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeepLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // deepLinkInfo
  /// Text to be shown to the user.
  text: Option<FormattedText>,
  /// True, if user should be asked to update the application.
  need_update_application: Option<bool>,
  
}



impl Object for DeepLinkInfo {}
impl RObject for DeepLinkInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "deepLinkInfo" }
  fn td_type(&self) -> RTDType { RTDType::DeepLinkInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl DeepLinkInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "deepLinkInfo".to_string(),
      text: None,
      need_update_application: None,
      
    }
  }
  
  pub fn text(&self) -> Option<FormattedText> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: FormattedText) -> &mut Self { self.text = Some(text); self }
  
  pub fn need_update_application(&self) -> Option<bool> { self.need_update_application.clone() }
  #[doc(hidden)] pub fn _set_need_update_application(&mut self, need_update_application: bool) -> &mut Self { self.need_update_application = Some(need_update_application); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



