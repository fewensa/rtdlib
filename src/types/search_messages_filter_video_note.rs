
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only video note messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterVideoNote {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterVideoNote
  
}



impl Object for SearchMessagesFilterVideoNote {}
impl RObject for SearchMessagesFilterVideoNote {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterVideoNote" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterVideoNote }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterVideoNote {}


impl SearchMessagesFilterVideoNote {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterVideoNote".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



