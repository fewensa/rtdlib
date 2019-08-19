
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Searches for recently used hashtags by their prefix.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHashtags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchHashtags
  /// Hashtag prefix to search for.
  prefix: Option<String>,
  /// Maximum number of hashtags to be returned.
  limit: Option<i32>,
  
}



impl Object for SearchHashtags {}
impl RObject for SearchHashtags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchHashtags" }
  fn td_type(&self) -> RTDType { RTDType::SearchHashtags }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for SearchHashtags {}


impl SearchHashtags {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchHashtags".to_string(),
      prefix: None,
      limit: None,
      
    }
  }
  
  pub fn prefix(&self) -> Option<String> { self.prefix.clone() }
  #[doc(hidden)] pub fn _set_prefix(&mut self, prefix: String) -> &mut Self { self.prefix = Some(prefix); self }
  
  pub fn limit(&self) -> Option<i32> { self.limit.clone() }
  #[doc(hidden)] pub fn _set_limit(&mut self, limit: i32) -> &mut Self { self.limit = Some(limit); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



