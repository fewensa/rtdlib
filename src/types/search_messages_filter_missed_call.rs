
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only incoming call messages with missed/declined discard reasons. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterMissedCall {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterMissedCall
  
}



impl Object for SearchMessagesFilterMissedCall {}
impl RObject for SearchMessagesFilterMissedCall {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterMissedCall" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterMissedCall }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterMissedCall {}


impl SearchMessagesFilterMissedCall {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterMissedCall".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



