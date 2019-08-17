
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains a list of available TDLib internal log tags. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogTags {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // logTags
  /// List of log tags.
  tags: Option<Vec<String>>,
  
}



impl Object for LogTags {}
impl RObject for LogTags {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "logTags" }
  fn td_type(&self) -> RTDType { RTDType::LogTags }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl LogTags {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "logTags".to_string(),
      tags: None,
      
    }
  }
  
  pub fn tags(&self) -> Option<Vec<String>> { self.tags.clone() }
  #[doc(hidden)] pub fn _set_tags(&mut self, tags: Vec<String>) -> &mut Self { self.tags = Some(tags); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



