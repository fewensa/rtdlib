
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only messages containing chat photos. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterChatPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterChatPhoto
  
}



impl Object for SearchMessagesFilterChatPhoto {}
impl RObject for SearchMessagesFilterChatPhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterChatPhoto" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterChatPhoto }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterChatPhoto {}


impl SearchMessagesFilterChatPhoto {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterChatPhoto".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



