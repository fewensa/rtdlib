
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Removes a hashtag from the list of recently used hashtags.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRecentHashtag {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // removeRecentHashtag
  /// Hashtag to delete.
  hashtag: Option<String>,
  
}



impl Object for RemoveRecentHashtag {}
impl RObject for RemoveRecentHashtag {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "removeRecentHashtag" }
  fn td_type(&self) -> RTDType { RTDType::RemoveRecentHashtag }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


impl Function for RemoveRecentHashtag {}


impl RemoveRecentHashtag {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "removeRecentHashtag".to_string(),
      hashtag: None,
      
    }
  }
  
  pub fn hashtag(&self) -> Option<String> { self.hashtag.clone() }
  #[doc(hidden)] pub fn _set_hashtag(&mut self, hashtag: String) -> &mut Self { self.hashtag = Some(hashtag); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



