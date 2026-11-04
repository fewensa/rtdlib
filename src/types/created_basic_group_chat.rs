use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a newly created basic group chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreatedBasicGroupChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// Information about failed to add members
  failed_to_add_members: FailedToAddMembers,
}

impl RObject for CreatedBasicGroupChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "createdBasicGroupChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CreatedBasicGroupChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCreatedBasicGroupChatBuilder {
    let inner = CreatedBasicGroupChat {
      td_name: "createdBasicGroupChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCreatedBasicGroupChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn failed_to_add_members(&self) -> &FailedToAddMembers {
    &self.failed_to_add_members
  }
}

#[doc(hidden)]
pub struct RTDCreatedBasicGroupChatBuilder {
  inner: CreatedBasicGroupChat,
}

impl RTDCreatedBasicGroupChatBuilder {
  pub fn build(&self) -> CreatedBasicGroupChat {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn failed_to_add_members<T: AsRef<FailedToAddMembers>>(
    &mut self,
    failed_to_add_members: T,
  ) -> &mut Self {
    self.inner.failed_to_add_members = failed_to_add_members.as_ref().clone();
    self
  }
}

impl AsRef<CreatedBasicGroupChat> for CreatedBasicGroupChat {
  fn as_ref(&self) -> &CreatedBasicGroupChat {
    self
  }
}

impl AsRef<CreatedBasicGroupChat> for RTDCreatedBasicGroupChatBuilder {
  fn as_ref(&self) -> &CreatedBasicGroupChat {
    &self.inner
  }
}
