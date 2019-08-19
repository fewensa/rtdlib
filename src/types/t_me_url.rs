
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Represents a URL linking to an internal Telegram entity. 
#[derive(Debug, Serialize, Deserialize)]
pub struct TMeUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // tMeUrl
  /// URL.
  url: Option<String>,
  /// Type of the URL.
  #[serde(rename(serialize = "type", deserialize = "type"))] type_: Option<Box<TMeUrlType>>,
  
}


impl Clone for TMeUrl {
  fn clone(&self) -> Self { rtd_clone!()(self) }
}


impl Object for TMeUrl {}
impl RObject for TMeUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "tMeUrl" }
  fn td_type(&self) -> RTDType { RTDType::TMeUrl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl TMeUrl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "tMeUrl".to_string(),
      url: None,
      type_: None,
      
    }
  }
  
  pub fn url(&self) -> Option<String> { self.url.clone() }
  #[doc(hidden)] pub fn _set_url(&mut self, url: String) -> &mut Self { self.url = Some(url); self }
  
  pub fn type_(&self) -> Option<Box<TMeUrlType>> { self.type_.clone() }
  #[doc(hidden)] pub fn _set_type_(&mut self, type_: Box<TMeUrlType>) -> &mut Self { self.type_ = Some(type_); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



