use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes actions that a user is allowed to take in a community
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityPermissions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the user can change the chats added to the community
  can_edit_chat_list: bool,
}

impl RObject for CommunityPermissions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityPermissions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CommunityPermissions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityPermissionsBuilder {
    let inner = CommunityPermissions {
      td_name: "communityPermissions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCommunityPermissionsBuilder { inner }
  }

  pub fn can_edit_chat_list(&self) -> bool {
    self.can_edit_chat_list
  }
}

#[doc(hidden)]
pub struct RTDCommunityPermissionsBuilder {
  inner: CommunityPermissions,
}

impl RTDCommunityPermissionsBuilder {
  pub fn build(&self) -> CommunityPermissions {
    self.inner.clone()
  }

  pub fn can_edit_chat_list(&mut self, can_edit_chat_list: bool) -> &mut Self {
    self.inner.can_edit_chat_list = can_edit_chat_list;
    self
  }
}

impl AsRef<CommunityPermissions> for CommunityPermissions {
  fn as_ref(&self) -> &CommunityPermissions {
    self
  }
}

impl AsRef<CommunityPermissions> for RTDCommunityPermissionsBuilder {
  fn as_ref(&self) -> &CommunityPermissions {
    &self.inner
  }
}
