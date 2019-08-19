
use std::fmt::Debug;
use std::str::FromStr;

use crate::types::*;
use crate::tdkit;

/// Contains information about the availability of the "Report spam" action for a chat. 
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatReportSpamState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String, // chatReportSpamState
  /// True, if a prompt with the "Report spam" action should be shown to the user.
  can_report_spam: Option<bool>,
  
}



impl Object for ChatReportSpamState {}
impl RObject for ChatReportSpamState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatReportSpamState" }
  fn td_type(&self) -> RTDType { RTDType::ChatReportSpamState }
  fn to_json(&self) -> String { rtd_to_json!()(self) }
}



impl ChatReportSpamState {
  #[doc(hidden)] pub fn _new() -> Self {
    Self {
      td_name: "chatReportSpamState".to_string(),
      can_report_spam: None,
      
    }
  }
  
  pub fn can_report_spam(&self) -> Option<bool> { self.can_report_spam.clone() }
  #[doc(hidden)] pub fn _set_can_report_spam(&mut self, can_report_spam: bool) -> &mut Self { self.can_report_spam = Some(can_report_spam); self }
  
  pub fn from_json<S: AsRef<str>>(json: S) -> Option<Self> { from_json!()(json.as_ref()) }
}



