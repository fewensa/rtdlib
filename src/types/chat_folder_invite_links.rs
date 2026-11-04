use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of chat folder invite links
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderInviteLinks {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of the invite links
  invite_links: Vec<ChatFolderInviteLink>,
}

impl RObject for ChatFolderInviteLinks {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatFolderInviteLinks"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatFolderInviteLinks {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatFolderInviteLinksBuilder {
    let inner = ChatFolderInviteLinks {
      td_name: "chatFolderInviteLinks".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatFolderInviteLinksBuilder { inner }
  }

  pub fn invite_links(&self) -> &Vec<ChatFolderInviteLink> {
    &self.invite_links
  }
}

#[doc(hidden)]
pub struct RTDChatFolderInviteLinksBuilder {
  inner: ChatFolderInviteLinks,
}

impl RTDChatFolderInviteLinksBuilder {
  pub fn build(&self) -> ChatFolderInviteLinks {
    self.inner.clone()
  }

  pub fn invite_links(&mut self, invite_links: Vec<ChatFolderInviteLink>) -> &mut Self {
    self.inner.invite_links = invite_links;
    self
  }
}

impl AsRef<ChatFolderInviteLinks> for ChatFolderInviteLinks {
  fn as_ref(&self) -> &ChatFolderInviteLinks {
    self
  }
}

impl AsRef<ChatFolderInviteLinks> for RTDChatFolderInviteLinksBuilder {
  fn as_ref(&self) -> &ChatFolderInviteLinks {
    &self.inner
  }
}
