
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns all found messages, no filter is applied. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterEmpty {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterEmpty
  
}



impl Object for SearchMessagesFilterEmpty {}
impl RObject for SearchMessagesFilterEmpty {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterEmpty" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterEmpty }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterEmpty {}


impl SearchMessagesFilterEmpty {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterEmpty".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



