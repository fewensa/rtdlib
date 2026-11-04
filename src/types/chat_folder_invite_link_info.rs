use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an invite link to a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderInviteLinkInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Basic information about the chat folder; chat folder identifier will be 0 if the user didn't have the chat folder yet
  chat_folder_info: ChatFolderInfo,
  /// Identifiers of the chats from the link, which aren't added to the folder yet
  missing_chat_ids: Vec<i64>,
  /// Identifiers of the chats from the link, which are added to the folder already
  added_chat_ids: Vec<i64>,
}

impl RObject for ChatFolderInviteLinkInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatFolderInviteLinkInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatFolderInviteLinkInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatFolderInviteLinkInfoBuilder {
    let inner = ChatFolderInviteLinkInfo {
      td_name: "chatFolderInviteLinkInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatFolderInviteLinkInfoBuilder { inner }
  }

  pub fn chat_folder_info(&self) -> &ChatFolderInfo {
    &self.chat_folder_info
  }

  pub fn missing_chat_ids(&self) -> &Vec<i64> {
    &self.missing_chat_ids
  }

  pub fn added_chat_ids(&self) -> &Vec<i64> {
    &self.added_chat_ids
  }
}

#[doc(hidden)]
pub struct RTDChatFolderInviteLinkInfoBuilder {
  inner: ChatFolderInviteLinkInfo,
}

impl RTDChatFolderInviteLinkInfoBuilder {
  pub fn build(&self) -> ChatFolderInviteLinkInfo {
    self.inner.clone()
  }

  pub fn chat_folder_info<T: AsRef<ChatFolderInfo>>(&mut self, chat_folder_info: T) -> &mut Self {
    self.inner.chat_folder_info = chat_folder_info.as_ref().clone();
    self
  }

  pub fn missing_chat_ids(&mut self, missing_chat_ids: Vec<i64>) -> &mut Self {
    self.inner.missing_chat_ids = missing_chat_ids;
    self
  }

  pub fn added_chat_ids(&mut self, added_chat_ids: Vec<i64>) -> &mut Self {
    self.inner.added_chat_ids = added_chat_ids;
    self
  }
}

impl AsRef<ChatFolderInviteLinkInfo> for ChatFolderInviteLinkInfo {
  fn as_ref(&self) -> &ChatFolderInviteLinkInfo {
    self
  }
}

impl AsRef<ChatFolderInviteLinkInfo> for RTDChatFolderInviteLinkInfoBuilder {
  fn as_ref(&self) -> &ChatFolderInviteLinkInfo {
    &self.inner
  }
}
