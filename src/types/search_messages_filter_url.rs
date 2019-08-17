
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only messages containing URLs. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterUrl
  
}



impl Object for SearchMessagesFilterUrl {}
impl RObject for SearchMessagesFilterUrl {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterUrl" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterUrl }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterUrl {}


impl SearchMessagesFilterUrl {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterUrl".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



