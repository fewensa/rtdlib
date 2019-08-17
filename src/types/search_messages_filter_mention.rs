
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only messages with mentions of the current user, or messages that are replies to their messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterMention {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterMention
  
}



impl Object for SearchMessagesFilterMention {}
impl RObject for SearchMessagesFilterMention {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterMention" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterMention }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterMention {}


impl SearchMessagesFilterMention {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterMention".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



