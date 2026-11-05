use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains full information about a community
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityFullInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo of the community
  photo: ChatPhoto,
  /// Chats belonging to the community
  chats: Vec<CommunityChat>,
  /// Number of privileged users in the community; 0 if the current user isn't an administrator of the community
  administrator_count: i64,
  /// Number of users banned from the community; 0 if the current user isn't an administrator of the community
  banned_count: i64,
  /// Number of pending requests for addition of chats to the community; 0 if the current user isn't an administrator of the community
  add_chat_request_count: i64,
}

impl RObject for CommunityFullInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityFullInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CommunityFullInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityFullInfoBuilder {
    let inner = CommunityFullInfo {
      td_name: "communityFullInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCommunityFullInfoBuilder { inner }
  }

  pub fn photo(&self) -> &ChatPhoto {
    &self.photo
  }

  pub fn chats(&self) -> &Vec<CommunityChat> {
    &self.chats
  }

  pub fn administrator_count(&self) -> i64 {
    self.administrator_count
  }

  pub fn banned_count(&self) -> i64 {
    self.banned_count
  }

  pub fn add_chat_request_count(&self) -> i64 {
    self.add_chat_request_count
  }
}

#[doc(hidden)]
pub struct RTDCommunityFullInfoBuilder {
  inner: CommunityFullInfo,
}

impl RTDCommunityFullInfoBuilder {
  pub fn build(&self) -> CommunityFullInfo {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<ChatPhoto>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

  pub fn chats(&mut self, chats: Vec<CommunityChat>) -> &mut Self {
    self.inner.chats = chats;
    self
  }

  pub fn administrator_count(&mut self, administrator_count: i64) -> &mut Self {
    self.inner.administrator_count = administrator_count;
    self
  }

  pub fn banned_count(&mut self, banned_count: i64) -> &mut Self {
    self.inner.banned_count = banned_count;
    self
  }

  pub fn add_chat_request_count(&mut self, add_chat_request_count: i64) -> &mut Self {
    self.inner.add_chat_request_count = add_chat_request_count;
    self
  }
}

impl AsRef<CommunityFullInfo> for CommunityFullInfo {
  fn as_ref(&self) -> &CommunityFullInfo {
    self
  }
}

impl AsRef<CommunityFullInfo> for RTDCommunityFullInfoBuilder {
  fn as_ref(&self) -> &CommunityFullInfo {
    &self.inner
  }
}
