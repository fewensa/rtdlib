use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a link to boost a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBoostLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the link will work for non-members of the chat
  is_public: bool,
  /// Identifier of the chat to which the link points; 0 if the chat isn't found
  chat_id: i64,
}

impl RObject for ChatBoostLinkInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBoostLinkInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBoostLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBoostLinkInfoBuilder {
    let inner = ChatBoostLinkInfo {
      td_name: "chatBoostLinkInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBoostLinkInfoBuilder { inner }
  }

  pub fn is_public(&self) -> bool {
    self.is_public
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDChatBoostLinkInfoBuilder {
  inner: ChatBoostLinkInfo,
}

impl RTDChatBoostLinkInfoBuilder {
  pub fn build(&self) -> ChatBoostLinkInfo {
    self.inner.clone()
  }

  pub fn is_public(&mut self, is_public: bool) -> &mut Self {
    self.inner.is_public = is_public;
    self
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<ChatBoostLinkInfo> for ChatBoostLinkInfo {
  fn as_ref(&self) -> &ChatBoostLinkInfo {
    self
  }
}

impl AsRef<ChatBoostLinkInfo> for RTDChatBoostLinkInfoBuilder {
  fn as_ref(&self) -> &ChatBoostLinkInfo {
    &self.inner
  }
}
