
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes a user that sent a join request and waits for administrator approval
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatJoinRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// User identifier
  user_id: i64,
  /// Point in time (Unix timestamp) when the user sent the join request
  date: i64,
  /// A short bio of the user
  bio: String,
  
}

impl RObject for ChatJoinRequest {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatJoinRequest" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatJoinRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatJoinRequestBuilder {
    let mut inner = ChatJoinRequest::default();
    inner.td_name = "chatJoinRequest".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatJoinRequestBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn date(&self) -> i64 { self.date }

  pub fn bio(&self) -> &String { &self.bio }

}

#[doc(hidden)]
pub struct RTDChatJoinRequestBuilder {
  inner: ChatJoinRequest
}

impl RTDChatJoinRequestBuilder {
  pub fn build(&self) -> ChatJoinRequest { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

   
  pub fn bio<T: AsRef<str>>(&mut self, bio: T) -> &mut Self {
    self.inner.bio = bio.as_ref().to_string();
    self
  }

}

impl AsRef<ChatJoinRequest> for ChatJoinRequest {
  fn as_ref(&self) -> &ChatJoinRequest { self }
}

impl AsRef<ChatJoinRequest> for RTDChatJoinRequestBuilder {
  fn as_ref(&self) -> &ChatJoinRequest { &self.inner }
}



