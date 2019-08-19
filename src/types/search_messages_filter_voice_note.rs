
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only voice note messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterVoiceNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterVoiceNote
  
}



impl Object for SearchMessagesFilterVoiceNote {}
impl RObject for SearchMessagesFilterVoiceNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterVoiceNote" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterVoiceNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterVoiceNote {}


impl SearchMessagesFilterVoiceNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterVoiceNote".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



