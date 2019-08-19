
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only voice and video note messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterVoiceAndVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterVoiceAndVideoNote
  
}



impl Object for SearchMessagesFilterVoiceAndVideoNote {}
impl RObject for SearchMessagesFilterVoiceAndVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterVoiceAndVideoNote" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterVoiceAndVideoNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterVoiceAndVideoNote {}


impl SearchMessagesFilterVoiceAndVideoNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterVoiceAndVideoNote".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



