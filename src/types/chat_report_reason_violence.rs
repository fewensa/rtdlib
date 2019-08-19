
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat promotes violence. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReportReasonViolence {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatReportReasonViolence
  
}



impl Object for ChatReportReasonViolence {}
impl RObject for ChatReportReasonViolence {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatReportReasonViolence" }
  fn td_type(&self) -> RTDType { RTDType::ChatReportReasonViolence }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatReportReason for ChatReportReasonViolence {}


impl ChatReportReasonViolence {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatReportReasonViolence".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



