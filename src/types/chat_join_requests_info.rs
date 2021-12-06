
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains information about pending chat join requests
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequestsInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of pending join requests
  total_count: i64,
  /// Identifiers of users sent the newest pending join requests
  user_ids: Vec<i64>,
  
}

impl RObject for ChatJoinRequestsInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatJoinRequestsInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatJoinRequestsInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatJoinRequestsInfoBuilder {
    let mut inner = ChatJoinRequestsInfo::default();
    inner.td_name = "chatJoinRequestsInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatJoinRequestsInfoBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn user_ids(&self) -> &Vec<i64> { &self.user_ids }

}

#[doc(hidden)]
pub struct RTDChatJoinRequestsInfoBuilder {
  inner: ChatJoinRequestsInfo
}

impl RTDChatJoinRequestsInfoBuilder {
  pub fn build(&self) -> ChatJoinRequestsInfo { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn user_ids(&mut self, user_ids: Vec<i64>) -> &mut Self {
    self.inner.user_ids = user_ids;
    self
  }

}

impl AsRef<ChatJoinRequestsInfo> for ChatJoinRequestsInfo {
  fn as_ref(&self) -> &ChatJoinRequestsInfo { self }
}

impl AsRef<ChatJoinRequestsInfo> for RTDChatJoinRequestsInfoBuilder {
  fn as_ref(&self) -> &ChatJoinRequestsInfo { &self.inner }
}



