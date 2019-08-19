
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// The chat has child abuse related content. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReportReasonChildAbuse {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatReportReasonChildAbuse
  
}



impl Object for ChatReportReasonChildAbuse {}
impl RObject for ChatReportReasonChildAbuse {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatReportReasonChildAbuse" }
  fn td_type(&self) -> RTDType { RTDType::ChatReportReasonChildAbuse }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatReportReason for ChatReportReasonChildAbuse {}


impl ChatReportReasonChildAbuse {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatReportReasonChildAbuse".to_string(),
      
    }
  }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



