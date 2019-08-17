
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Returns only animation messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchMessagesFilterAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // searchMessagesFilterAnimation
  
}



impl Object for SearchMessagesFilterAnimation {}
impl RObject for SearchMessagesFilterAnimation {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "searchMessagesFilterAnimation" }
  fn td_type(&self) -> RTDType { RTDType::SearchMessagesFilterAnimation }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl SearchMessagesFilter for SearchMessagesFilterAnimation {}


impl SearchMessagesFilterAnimation {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "searchMessagesFilterAnimation".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



