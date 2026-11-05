use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains basic information about a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique chat folder identifier
  id: i64,
  /// The name of the folder
  name: ChatFolderName,
  /// The chosen or default icon for the chat folder
  icon: ChatFolderIcon,
  /// The identifier of the chosen color for the chat folder icon; from 1 to 6. If 1, then color is disabled
  color_id: i64,
  /// True, if at least one link has been created for the folder
  is_shareable: bool,
  /// True, if the chat folder has invite links created by the current user
  has_my_invite_links: bool,
}

impl RObject for ChatFolderInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatFolderInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatFolderInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatFolderInfoBuilder {
    let inner = ChatFolderInfo {
      td_name: "chatFolderInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatFolderInfoBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn name(&self) -> &ChatFolderName {
    &self.name
  }

  pub fn icon(&self) -> &ChatFolderIcon {
    &self.icon
  }

  pub fn color_id(&self) -> i64 {
    self.color_id
  }

  pub fn is_shareable(&self) -> bool {
    self.is_shareable
  }

  pub fn has_my_invite_links(&self) -> bool {
    self.has_my_invite_links
  }
}

#[doc(hidden)]
pub struct RTDChatFolderInfoBuilder {
  inner: ChatFolderInfo,
}

impl RTDChatFolderInfoBuilder {
  pub fn build(&self) -> ChatFolderInfo {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn name<T: AsRef<ChatFolderName>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().clone();
    self
  }

  pub fn icon<T: AsRef<ChatFolderIcon>>(&mut self, icon: T) -> &mut Self {
    self.inner.icon = icon.as_ref().clone();
    self
  }

  pub fn color_id(&mut self, color_id: i64) -> &mut Self {
    self.inner.color_id = color_id;
    self
  }

  pub fn is_shareable(&mut self, is_shareable: bool) -> &mut Self {
    self.inner.is_shareable = is_shareable;
    self
  }

  pub fn has_my_invite_links(&mut self, has_my_invite_links: bool) -> &mut Self {
    self.inner.has_my_invite_links = has_my_invite_links;
    self
  }
}

impl AsRef<ChatFolderInfo> for ChatFolderInfo {
  fn as_ref(&self) -> &ChatFolderInfo {
    self
  }
}

impl AsRef<ChatFolderInfo> for RTDChatFolderInfoBuilder {
  fn as_ref(&self) -> &ChatFolderInfo {
    &self.inner
  }
}
