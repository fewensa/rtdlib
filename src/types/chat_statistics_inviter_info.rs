
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains statistics about number of new members invited by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsInviterInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// Number of new members invited by the user
  added_member_count: i64,
  
}

impl RObject for ChatStatisticsInviterInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsInviterInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatisticsInviterInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsInviterInfoBuilder {
    let mut inner = ChatStatisticsInviterInfo::default();
    inner.td_name = "chatStatisticsInviterInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatStatisticsInviterInfoBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn added_member_count(&self) -> i64 { self.added_member_count }

}

#[doc(hidden)]
pub struct RTDChatStatisticsInviterInfoBuilder {
  inner: ChatStatisticsInviterInfo
}

impl RTDChatStatisticsInviterInfoBuilder {
  pub fn build(&self) -> ChatStatisticsInviterInfo { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn added_member_count(&mut self, added_member_count: i64) -> &mut Self {
    self.inner.added_member_count = added_member_count;
    self
  }

}

impl AsRef<ChatStatisticsInviterInfo> for ChatStatisticsInviterInfo {
  fn as_ref(&self) -> &ChatStatisticsInviterInfo { self }
}

impl AsRef<ChatStatisticsInviterInfo> for RTDChatStatisticsInviterInfoBuilder {
  fn as_ref(&self) -> &ChatStatisticsInviterInfo { &self.inner }
}



