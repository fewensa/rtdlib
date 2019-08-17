
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A forwarded messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushMessageContentMessageForwards {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // pushMessageContentMessageForwards
  /// Number of forwarded messages.
  total_count: Option<i32>,
  
}



impl Object for PushMessageContentMessageForwards {}
impl RObject for PushMessageContentMessageForwards {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "pushMessageContentMessageForwards" }
  fn td_type(&self) -> RTDType { RTDType::PushMessageContentMessageForwards }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl PushMessageContent for PushMessageContentMessageForwards {}


impl PushMessageContentMessageForwards {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "pushMessageContentMessageForwards".to_string(),
      total_count: None,
      
    }
  }
  
  pub fn total_count(&self) -> Option<i32> { self.total_count.clone() }
  #[doc(hidden)] pub fn _set_total_count(&mut self, total_count: i32) -> &mut Self { self.total_count = Some(total_count); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



