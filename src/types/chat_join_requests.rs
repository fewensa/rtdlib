use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of requests to join a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequests {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of requests found
  total_count: i64,
  /// List of the requests
  requests: Vec<ChatJoinRequest>,
}

impl RObject for ChatJoinRequests {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatJoinRequests"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatJoinRequests {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatJoinRequestsBuilder {
    let inner = ChatJoinRequests {
      td_name: "chatJoinRequests".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatJoinRequestsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn requests(&self) -> &Vec<ChatJoinRequest> {
    &self.requests
  }
}

#[doc(hidden)]
pub struct RTDChatJoinRequestsBuilder {
  inner: ChatJoinRequests,
}

impl RTDChatJoinRequestsBuilder {
  pub fn build(&self) -> ChatJoinRequests {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn requests(&mut self, requests: Vec<ChatJoinRequest>) -> &mut Self {
    self.inner.requests = requests;
    self
  }
}

impl AsRef<ChatJoinRequests> for ChatJoinRequests {
  fn as_ref(&self) -> &ChatJoinRequests {
    self
  }
}

impl AsRef<ChatJoinRequests> for RTDChatJoinRequestsBuilder {
  fn as_ref(&self) -> &ChatJoinRequests {
    &self.inner
  }
}
