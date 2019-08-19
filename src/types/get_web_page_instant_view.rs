
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns an instant view version of a web page if available. Returns a 404 error if the web page has no instant view page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWebPageInstantView {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // getWebPageInstantView
  /// The web page URL.
  url: Option<String>,
  /// If true, the full instant view for the web page will be returned.
  force_full: Option<bool>,
  
}



impl Object for GetWebPageInstantView {}
impl RObject for GetWebPageInstantView {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "getWebPageInstantView" }
  fn td_type(&self) -> RTDType { RTDType::GetWebPageInstantView }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for GetWebPageInstantView {}


impl GetWebPageInstantView {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "getWebPageInstantView".to_string(),
      url: None,
      force_full: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn force_full(&self) -> Option<bool> { self.force_full.clone() }
  #[doc(hidden)] pub fn _set_force_full(&mut self, force_full: bool) -> &mut Self { self.force_full = Some(force_full); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



