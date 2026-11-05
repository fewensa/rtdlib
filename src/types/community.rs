use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a community consisting of supergroup chats, channel chats and chats with bots
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Community {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Community identifier
  id: i64,
  /// If false, the community is inaccessible, and the only information known about the community is inside this class. Identifier of the community can't be passed to any method
  have_access: bool,
  /// Community name
  name: String,
  /// Community photo; may be null
  photo: Option<ChatPhotoInfo>,
  /// Point in time (Unix timestamp) when the community was joined, or the point in time when the community was created, in case the user is not a member of any chat in the community
  date: i64,
  /// Status of the current user in the community
  status: CommunityMemberStatus,
  /// Actions that non-administrator community members are allowed to take in the community
  permissions: CommunityPermissions,
}

impl RObject for Community {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "community"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Community {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityBuilder {
    let inner = Community {
      td_name: "community".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCommunityBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn have_access(&self) -> bool {
    self.have_access
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn photo(&self) -> &Option<ChatPhotoInfo> {
    &self.photo
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn status(&self) -> &CommunityMemberStatus {
    &self.status
  }

  pub fn permissions(&self) -> &CommunityPermissions {
    &self.permissions
  }
}

#[doc(hidden)]
pub struct RTDCommunityBuilder {
  inner: Community,
}

impl RTDCommunityBuilder {
  pub fn build(&self) -> Community {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn have_access(&mut self, have_access: bool) -> &mut Self {
    self.inner.have_access = have_access;
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn photo<T: AsRef<ChatPhotoInfo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn status<T: AsRef<CommunityMemberStatus>>(&mut self, status: T) -> &mut Self {
    self.inner.status = status.as_ref().clone();
    self
  }

  pub fn permissions<T: AsRef<CommunityPermissions>>(&mut self, permissions: T) -> &mut Self {
    self.inner.permissions = permissions.as_ref().clone();
    self
  }
}

impl AsRef<Community> for Community {
  fn as_ref(&self) -> &Community {
    self
  }
}

impl AsRef<Community> for RTDCommunityBuilder {
  fn as_ref(&self) -> &Community {
    &self.inner
  }
}
