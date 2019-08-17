
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat contains copyrighted content. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReportReasonCopyright {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatReportReasonCopyright
  
}



impl Object for ChatReportReasonCopyright {}
impl RObject for ChatReportReasonCopyright {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatReportReasonCopyright" }
  fn td_type(&self) -> RTDType { RTDType::ChatReportReasonCopyright }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatReportReason for ChatReportReasonCopyright {}


impl ChatReportReasonCopyright {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatReportReasonCopyright".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



