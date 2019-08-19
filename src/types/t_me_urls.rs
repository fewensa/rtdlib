
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of t.me URLs. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TMeUrls {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // tMeUrls
  /// List of URLs.
  urls: Option<Vec<TMeUrl>>,
  
}



impl Object for TMeUrls {}
impl RObject for TMeUrls {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrls" }
  fn td_type(&self) -> RTDType { RTDType::TMeUrls }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TMeUrls {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "tMeUrls".to_string(),
      urls: None,
      
    }
  }
  
  pub fn urls(&self) -> Option<Vec<TMeUrl>> { self.urls.clone() }
  #[doc(hidden)] pub fn _set_urls(&mut self, urls: Vec<TMeUrl>) -> &mut Self { self.urls = Some(urls); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



