use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents an icon for a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderIcon {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The chosen icon name for short folder representation; one of "All", "Unread", "Unmuted", "Bots", "Channels", "Groups", "Private", "Custom", "Setup", "Cat", "Crown", "Favorite", "Flower", "Game", "Home", "Love", "Mask", "Party", "Sport", "Study", "Trade", "Travel", "Work", "Airplane", "Book", "Light", "Like", "Money", "Note", "Palette"
  name: String,
}

impl RObject for ChatFolderIcon {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatFolderIcon"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatFolderIcon {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatFolderIconBuilder {
    let inner = ChatFolderIcon {
      td_name: "chatFolderIcon".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatFolderIconBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }
}

#[doc(hidden)]
pub struct RTDChatFolderIconBuilder {
  inner: ChatFolderIcon,
}

impl RTDChatFolderIconBuilder {
  pub fn build(&self) -> ChatFolderIcon {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }
}

impl AsRef<ChatFolderIcon> for ChatFolderIcon {
  fn as_ref(&self) -> &ChatFolderIcon {
    self
  }
}

impl AsRef<ChatFolderIcon> for RTDChatFolderIconBuilder {
  fn as_ref(&self) -> &ChatFolderIcon {
    &self.inner
  }
}
