
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of hashtags. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hashtags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // hashtags
  /// A list of hashtags.
  hashtags: Option<Vec<String>>,
  
}



impl Object for Hashtags {}
impl RObject for Hashtags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "hashtags" }
  fn td_type(&self) -> RTDType { RTDType::Hashtags }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl Hashtags {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "hashtags".to_string(),
      hashtags: None,
      
    }
  }
  
  pub fn hashtags(&self) -> Option<Vec<String>> { self.hashtags.clone() }
  #[doc(hidden)] pub fn _set_hashtags(&mut self, hashtags: Vec<String>) -> &mut Self { self.hashtags = Some(hashtags); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



