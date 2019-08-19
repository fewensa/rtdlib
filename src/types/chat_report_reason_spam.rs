
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat contains spam messages. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReportReasonSpam {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatReportReasonSpam
  
}



impl Object for ChatReportReasonSpam {}
impl RObject for ChatReportReasonSpam {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatReportReasonSpam" }
  fn td_type(&self) -> RTDType { RTDType::ChatReportReasonSpam }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatReportReason for ChatReportReasonSpam {}


impl ChatReportReasonSpam {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatReportReasonSpam".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



