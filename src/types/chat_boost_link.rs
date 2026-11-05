use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains an HTTPS link to boost a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The link
  link: String,
  /// True, if the link will work for non-members of the chat
  is_public: bool,
}

impl RObject for ChatBoostLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoostLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostLinkBuilder {
    let inner = ChatBoostLink {
      td_name: "chatBoostLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostLinkBuilder { inner }
  }

  pub fn link(&self) -> &String {
    &self.link
  }

  pub fn is_public(&self) -> bool {
    self.is_public
  }
}

#[doc(hidden)]
pub struct RTDChatBoostLinkBuilder {
  inner: ChatBoostLink,
}

impl RTDChatBoostLinkBuilder {
  pub fn build(&self) -> ChatBoostLink {
    self.inner.clone()
  }

  pub fn link<T: AsRef<str>>(&mut self, link: T) -> &mut Self {
    self.inner.link = link.as_ref().to_string();
    self
  }

  pub fn is_public(&mut self, is_public: bool) -> &mut Self {
    self.inner.is_public = is_public;
    self
  }
}

impl AsRef<ChatBoostLink> for ChatBoostLink {
  fn as_ref(&self) -> &ChatBoostLink {
    self
  }
}

impl AsRef<ChatBoostLink> for RTDChatBoostLinkBuilder {
  fn as_ref(&self) -> &ChatBoostLink {
    &self.inner
  }
}
