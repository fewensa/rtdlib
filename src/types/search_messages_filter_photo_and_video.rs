
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only photo and video messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterPhotoAndVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterPhotoAndVideo
  
}



impl Object for SearchMessagesFilterPhotoAndVideo {}
impl RObject for SearchMessagesFilterPhotoAndVideo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterPhotoAndVideo" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterPhotoAndVideo }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterPhotoAndVideo {}


impl SearchMessagesFilterPhotoAndVideo {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterPhotoAndVideo".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



