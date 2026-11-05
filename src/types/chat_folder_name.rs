use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes name of a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderName {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The text of the chat folder name; 1-12 characters without line feeds. May contain only CustomEmoji entities
  text: FormattedText,
  /// True, if custom emoji in the name must be animated
  animate_custom_emoji: bool,
}

impl RObject for ChatFolderName {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatFolderName"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatFolderName {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatFolderNameBuilder {
    let inner = ChatFolderName {
      td_name: "chatFolderName".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatFolderNameBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn animate_custom_emoji(&self) -> bool {
    self.animate_custom_emoji
  }
}

#[doc(hidden)]
pub struct RTDChatFolderNameBuilder {
  inner: ChatFolderName,
}

impl RTDChatFolderNameBuilder {
  pub fn build(&self) -> ChatFolderName {
    self.inner.clone()
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn animate_custom_emoji(&mut self, animate_custom_emoji: bool) -> &mut Self {
    self.inner.animate_custom_emoji = animate_custom_emoji;
    self
  }
}

impl AsRef<ChatFolderName> for ChatFolderName {
  fn as_ref(&self) -> &ChatFolderName {
    self
  }
}

impl AsRef<ChatFolderName> for RTDChatFolderNameBuilder {
  fn as_ref(&self) -> &ChatFolderName {
    &self.inner
  }
}
