
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only messages with unread mentions of the current user, or messages that are replies to their messages. When using this filter the results can't be additionally filtered by a query or by the sending user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterUnreadMention {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterUnreadMention
  
}



impl Object for SearchMessagesFilterUnreadMention {}
impl RObject for SearchMessagesFilterUnreadMention {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterUnreadMention" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterUnreadMention }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterUnreadMention {}


impl SearchMessagesFilterUnreadMention {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterUnreadMention".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



