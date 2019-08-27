
use crate::types::*;
use crate::errors::*;




/// Contains information about the availability of the "Report spam" action for a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatReportSpamState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// True, if a prompt with the "Report spam" action should be shown to the user
  can_report_spam: bool,
  
}

impl RObject for ChatReportSpamState {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatReportSpamState" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatReportSpamState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatReportSpamStateBuilder {
    let mut inner = ChatReportSpamState::default();
    inner.td_name = "chatReportSpamState".to_string();
    RTDChatReportSpamStateBuilder { inner }
  }

  pub fn can_report_spam(&self) -> bool { self.can_report_spam }

}

#[doc(hidden)]
pub struct RTDChatReportSpamStateBuilder {
  inner: ChatReportSpamState
}

impl RTDChatReportSpamStateBuilder {
  pub fn build(&self) -> ChatReportSpamState { self.inner.clone() }

   
  pub fn can_report_spam(&mut self, can_report_spam: bool) -> &mut Self {
    self.inner.can_report_spam = can_report_spam;
    self
  }

}

impl AsRef<ChatReportSpamState> for ChatReportSpamState {
  fn as_ref(&self) -> &ChatReportSpamState { self }
}

impl AsRef<ChatReportSpamState> for RTDChatReportSpamStateBuilder {
  fn as_ref(&self) -> &ChatReportSpamState { &self.inner }
}



