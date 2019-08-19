
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// A custom reason provided by the user. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReportReasonCustom {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatReportReasonCustom
  /// Report text.
  text: Option<String>,
  
}



impl Object for ChatReportReasonCustom {}
impl RObject for ChatReportReasonCustom {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatReportReasonCustom" }
  fn td_type(&self) -> RTDType { RTDType::ChatReportReasonCustom }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}


#[typetag::serde] impl ChatReportReason for ChatReportReasonCustom {}


impl ChatReportReasonCustom {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatReportReasonCustom".to_string(),
      text: None,
      
    }
  }
  
  pub fn text(&self) -> Option<String> { self.text.clone() }
  #[doc(hidden)] pub fn _set_text(&mut self, text: String) -> &mut Self { self.text = Some(text); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



