
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains an HTTP URL. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // httpUrl
  /// The URL.
  url: Option<String>,
  
}



impl Object for HttpUrl {}
impl RObject for HttpUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "httpUrl" }
  fn td_type(&self) -> RTDType { RTDType::HttpUrl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl HttpUrl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "httpUrl".to_string(),
      url: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



