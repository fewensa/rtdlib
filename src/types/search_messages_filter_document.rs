
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only document messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterDocument {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterDocument
  
}



impl Object for SearchMessagesFilterDocument {}
impl RObject for SearchMessagesFilterDocument {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterDocument" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterDocument }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterDocument {}


impl SearchMessagesFilterDocument {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterDocument".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



