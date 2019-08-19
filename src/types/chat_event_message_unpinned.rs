
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A message was unpinned. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatEventMessageUnpinned {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatEventMessageUnpinned
  
}



impl Object for ChatEventMessageUnpinned {}
impl RObject for ChatEventMessageUnpinned {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatEventMessageUnpinned" }
  fn td_type(&self) -> RTDType { RTDType::ChatEventMessageUnpinned }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatEventAction for ChatEventMessageUnpinned {}


impl ChatEventMessageUnpinned {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatEventMessageUnpinned".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



