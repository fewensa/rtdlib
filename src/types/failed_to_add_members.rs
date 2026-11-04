use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of users that has failed to be added to a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FailedToAddMembers {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about users that weren't added to the chat
  failed_to_add_members: Vec<FailedToAddMember>,
}

impl RObject for FailedToAddMembers {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "failedToAddMembers"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl FailedToAddMembers {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDFailedToAddMembersBuilder {
    let inner = FailedToAddMembers {
      td_name: "failedToAddMembers".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDFailedToAddMembersBuilder { inner }
  }

  pub fn failed_to_add_members(&self) -> &Vec<FailedToAddMember> {
    &self.failed_to_add_members
  }
}

#[doc(hidden)]
pub struct RTDFailedToAddMembersBuilder {
  inner: FailedToAddMembers,
}

impl RTDFailedToAddMembersBuilder {
  pub fn build(&self) -> FailedToAddMembers {
    self.inner.clone()
  }

  pub fn failed_to_add_members(
    &mut self,
    failed_to_add_members: Vec<FailedToAddMember>,
  ) -> &mut Self {
    self.inner.failed_to_add_members = failed_to_add_members;
    self
  }
}

impl AsRef<FailedToAddMembers> for FailedToAddMembers {
  fn as_ref(&self) -> &FailedToAddMembers {
    self
  }
}

impl AsRef<FailedToAddMembers> for RTDFailedToAddMembersBuilder {
  fn as_ref(&self) -> &FailedToAddMembers {
    &self.inner
  }
}
