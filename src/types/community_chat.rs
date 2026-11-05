use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat in a community
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CommunityChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat in the community
  chat_id: i64,
  /// True, if message history of the chat can be viewed
  can_view_history: bool,
  /// True, if the chat is hidden in the list of community chats; for community administrators only
  is_hidden: bool,
}

impl RObject for CommunityChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "communityChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl CommunityChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDCommunityChatBuilder {
    let inner = CommunityChat {
      td_name: "communityChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDCommunityChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }

  pub fn can_view_history(&self) -> bool {
    self.can_view_history
  }

  pub fn is_hidden(&self) -> bool {
    self.is_hidden
  }
}

#[doc(hidden)]
pub struct RTDCommunityChatBuilder {
  inner: CommunityChat,
}

impl RTDCommunityChatBuilder {
  pub fn build(&self) -> CommunityChat {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

  pub fn can_view_history(&mut self, can_view_history: bool) -> &mut Self {
    self.inner.can_view_history = can_view_history;
    self
  }

  pub fn is_hidden(&mut self, is_hidden: bool) -> &mut Self {
    self.inner.is_hidden = is_hidden;
    self
  }
}

impl AsRef<CommunityChat> for CommunityChat {
  fn as_ref(&self) -> &CommunityChat {
    self
  }
}

impl AsRef<CommunityChat> for RTDCommunityChatBuilder {
  fn as_ref(&self) -> &CommunityChat {
    &self.inner
  }
}
