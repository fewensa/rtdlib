
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only audio messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterAudio {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterAudio
  
}



impl Object for SearchMessagesFilterAudio {}
impl RObject for SearchMessagesFilterAudio {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterAudio" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterAudio }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterAudio {}


impl SearchMessagesFilterAudio {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterAudio".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



