
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Describes an instant view page for a web page. 
#[derive(Debug, Serialize, Deserialize)]
pub struct WebPageInstantView {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // webPageInstantView
  /// Content of the web page.
  page_blocks: Option<Vec<Box<PageBlock>>>,
  /// Version of the instant view, currently can be 1 or 2.
  version: Option<i32>,
  /// Instant view URL; may be different from WebPage.url and must be used for the correct anchors handling.
  url: Option<String>,
  /// True, if the instant view must be shown from right to left.
  is_rtl: Option<bool>,
  /// True, if the instant view contains the full page. A network request might be needed to get the full web page instant view.
  is_full: Option<bool>,
  
}


impl Clone for WebPageInstantView {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for WebPageInstantView {}
impl RObject for WebPageInstantView {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "webPageInstantView" }
  fn td_type(&self) -> RTDType { RTDType::WebPageInstantView }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl WebPageInstantView {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "webPageInstantView".to_string(),
      page_blocks: None,
      version: None,
      url: None,
      is_rtl: None,
      is_full: None,
      
    }
  }
  
  pub fn page_blocks(&self) -> Option<Vec<Box<PageBlock>>> { self.page_blocks.clone() }
  #[doc(hidden)] pub fn _set_page_blocks(&mut self, page_blocks: Vec<Box<PageBlock>>) -> &mut Self { self.page_blocks = Some(page_blocks); self }
  
  pub fn version(&self) -> Option<i32> { self.version.clone() }
  #[doc(hidden)] pub fn _set_version(&mut self, version: i32) -> &mut Self { self.version = Some(version); self }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn is_rtl(&self) -> Option<bool> { self.is_rtl.clone() }
  #[doc(hidden)] pub fn _set_is_rtl(&mut self, is_rtl: bool) -> &mut Self { self.is_rtl = Some(is_rtl); self }
  
  pub fn is_full(&self) -> Option<bool> { self.is_full.clone() }
  #[doc(hidden)] pub fn _set_is_full(&mut self, is_full: bool) -> &mut Self { self.is_full = Some(is_full); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



