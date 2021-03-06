
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains statistics about administrator actions done by a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatStatisticsAdministratorActionsInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Administrator user identifier
  user_id: i64,
  /// Number of messages deleted by the administrator
  deleted_message_count: i64,
  /// Number of users banned by the administrator
  banned_user_count: i64,
  /// Number of users restricted by the administrator
  restricted_user_count: i64,
  
}

impl RObject for ChatStatisticsAdministratorActionsInfo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatStatisticsAdministratorActionsInfo" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatStatisticsAdministratorActionsInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatStatisticsAdministratorActionsInfoBuilder {
    let mut inner = ChatStatisticsAdministratorActionsInfo::default();
    inner.td_name = "chatStatisticsAdministratorActionsInfo".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatStatisticsAdministratorActionsInfoBuilder { inner }
  }

  pub fn user_id(&self) -> i64 { self.user_id }

  pub fn deleted_message_count(&self) -> i64 { self.deleted_message_count }

  pub fn banned_user_count(&self) -> i64 { self.banned_user_count }

  pub fn restricted_user_count(&self) -> i64 { self.restricted_user_count }

}

#[doc(hidden)]
pub struct RTDChatStatisticsAdministratorActionsInfoBuilder {
  inner: ChatStatisticsAdministratorActionsInfo
}

impl RTDChatStatisticsAdministratorActionsInfoBuilder {
  pub fn build(&self) -> ChatStatisticsAdministratorActionsInfo { self.inner.clone() }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

   
  pub fn deleted_message_count(&mut self, deleted_message_count: i64) -> &mut Self {
    self.inner.deleted_message_count = deleted_message_count;
    self
  }

   
  pub fn banned_user_count(&mut self, banned_user_count: i64) -> &mut Self {
    self.inner.banned_user_count = banned_user_count;
    self
  }

   
  pub fn restricted_user_count(&mut self, restricted_user_count: i64) -> &mut Self {
    self.inner.restricted_user_count = restricted_user_count;
    self
  }

}

impl AsRef<ChatStatisticsAdministratorActionsInfo> for ChatStatisticsAdministratorActionsInfo {
  fn as_ref(&self) -> &ChatStatisticsAdministratorActionsInfo { self }
}

impl AsRef<ChatStatisticsAdministratorActionsInfo> for RTDChatStatisticsAdministratorActionsInfoBuilder {
  fn as_ref(&self) -> &ChatStatisticsAdministratorActionsInfo { &self.inner }
}



