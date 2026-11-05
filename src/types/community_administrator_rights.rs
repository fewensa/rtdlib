use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes rights of the administrator in a community
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityAdministratorRights {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the user is an administrator. Implied by any other privilege
  can_manage_community: bool,
  /// True, if the administrator can change the community name, photo, and other settings
  can_change_info: bool,
  /// True, if the user can change the chats added to the community
  can_edit_chat_list: bool,
  /// True, if the administrator can add new administrators with a subset of their own privileges or demote administrators that were directly or indirectly promoted by them
  can_promote_members: bool,
  /// True, if the administrator can ban, or unban community members
  can_ban_members: bool,
}

impl RObject for CommunityAdministratorRights {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityAdministratorRights"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CommunityAdministratorRights {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityAdministratorRightsBuilder {
    let inner = CommunityAdministratorRights {
      td_name: "communityAdministratorRights".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCommunityAdministratorRightsBuilder { inner }
  }

  pub fn can_manage_community(&self) -> bool {
    self.can_manage_community
  }

  pub fn can_change_info(&self) -> bool {
    self.can_change_info
  }

  pub fn can_edit_chat_list(&self) -> bool {
    self.can_edit_chat_list
  }

  pub fn can_promote_members(&self) -> bool {
    self.can_promote_members
  }

  pub fn can_ban_members(&self) -> bool {
    self.can_ban_members
  }
}

#[doc(hidden)]
pub struct RTDCommunityAdministratorRightsBuilder {
  inner: CommunityAdministratorRights,
}

impl RTDCommunityAdministratorRightsBuilder {
  pub fn build(&self) -> CommunityAdministratorRights {
    self.inner.clone()
  }

  pub fn can_manage_community(&mut self, can_manage_community: bool) -> &mut Self {
    self.inner.can_manage_community = can_manage_community;
    self
  }

  pub fn can_change_info(&mut self, can_change_info: bool) -> &mut Self {
    self.inner.can_change_info = can_change_info;
    self
  }

  pub fn can_edit_chat_list(&mut self, can_edit_chat_list: bool) -> &mut Self {
    self.inner.can_edit_chat_list = can_edit_chat_list;
    self
  }

  pub fn can_promote_members(&mut self, can_promote_members: bool) -> &mut Self {
    self.inner.can_promote_members = can_promote_members;
    self
  }

  pub fn can_ban_members(&mut self, can_ban_members: bool) -> &mut Self {
    self.inner.can_ban_members = can_ban_members;
    self
  }
}

impl AsRef<CommunityAdministratorRights> for CommunityAdministratorRights {
  fn as_ref(&self) -> &CommunityAdministratorRights {
    self
  }
}

impl AsRef<CommunityAdministratorRights> for RTDCommunityAdministratorRightsBuilder {
  fn as_ref(&self) -> &CommunityAdministratorRights {
    &self.inner
  }
}
