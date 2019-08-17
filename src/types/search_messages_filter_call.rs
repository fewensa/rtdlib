
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only call messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterCall
  
}



impl Object for SearchMessagesFilterCall {}
impl RObject for SearchMessagesFilterCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterCall" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterCall {}


impl SearchMessagesFilterCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterCall".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



