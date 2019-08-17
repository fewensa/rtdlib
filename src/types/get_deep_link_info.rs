
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns information about a tg:// deep link. Use "tg://need_update_for_some_feature" or "tg:some_unsupported_feature" for testing. Returns a 404 error for unknown links. Can be called before authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDeepLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getDeepLinkInfo
  /// The link.
  link: Option<String>,
  
}



impl Object for GetDeepLinkInfo {}
impl RObject for GetDeepLinkInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getDeepLinkInfo" }
  fn td_type(&self) -> RTDType { RTDType::GetDeepLinkInfo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetDeepLinkInfo {}


impl GetDeepLinkInfo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getDeepLinkInfo".to_string(),
      link: None,
      
    }
  }
  
  pub fn link(&self) -> Option<String> { self.link.clone() }
  #[doc(hidden)] pub fn _set_link(&mut self, link: String) -> &mut Self { self.link = Some(link); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



