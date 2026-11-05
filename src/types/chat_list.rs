use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a list of chats
pub trait TDChatList: Debug + RObject {}

/// Describes a list of chats
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatList {
  #[doc(hidden)]
  _Default(()),
  /// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
  Archive(ChatListArchive),
  /// A list of chats added to a chat folder
  Folder(ChatListFolder),
  /// A main list of chats
  Main(ChatListMain),
}

impl Default for ChatList {
  fn default() -> Self {
    ChatList::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatList {
  fn deserialize<D>(deserializer: D) -> Result<ChatList, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatList,
      (chatListArchive, Archive);
      (chatListFolder, Folder);
      (chatListMain, Main);

    )(deserializer)
  }
}

impl RObject for ChatList {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatList::Archive(t) => t.td_name(),
      ChatList::Folder(t) => t.td_name(),
      ChatList::Main(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatList::Archive(t) => t.extra(),
      ChatList::Folder(t) => t.extra(),
      ChatList::Main(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatList::_Default(_))
  }

  pub fn is_archive(&self) -> bool {
    matches!(self, ChatList::Archive(_))
  }
  pub fn is_folder(&self) -> bool {
    matches!(self, ChatList::Folder(_))
  }
  pub fn is_main(&self) -> bool {
    matches!(self, ChatList::Main(_))
  }

  pub fn on_archive<F: FnOnce(&ChatListArchive)>(&self, fnc: F) -> &Self {
    if let ChatList::Archive(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_folder<F: FnOnce(&ChatListFolder)>(&self, fnc: F) -> &Self {
    if let ChatList::Folder(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_main<F: FnOnce(&ChatListMain)>(&self, fnc: F) -> &Self {
    if let ChatList::Main(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_archive(&self) -> Option<&ChatListArchive> {
    if let ChatList::Archive(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_folder(&self) -> Option<&ChatListFolder> {
    if let ChatList::Folder(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_main(&self) -> Option<&ChatListMain> {
    if let ChatList::Main(t) = self {
      return Some(t);
    }
    None
  }

  pub fn archive<T: AsRef<ChatListArchive>>(t: T) -> Self {
    ChatList::Archive(t.as_ref().clone())
  }

  pub fn folder<T: AsRef<ChatListFolder>>(t: T) -> Self {
    ChatList::Folder(t.as_ref().clone())
  }

  pub fn main<T: AsRef<ChatListMain>>(t: T) -> Self {
    ChatList::Main(t.as_ref().clone())
  }
}

impl AsRef<ChatList> for ChatList {
  fn as_ref(&self) -> &ChatList {
    self
  }
}

/// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListArchive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatListArchive {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatListArchive"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatList for ChatListArchive {}

impl ChatListArchive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatListArchiveBuilder {
    let inner = ChatListArchive {
      td_name: "chatListArchive".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatListArchiveBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatListArchiveBuilder {
  inner: ChatListArchive,
}

impl RTDChatListArchiveBuilder {
  pub fn build(&self) -> ChatListArchive {
    self.inner.clone()
  }
}

impl AsRef<ChatListArchive> for ChatListArchive {
  fn as_ref(&self) -> &ChatListArchive {
    self
  }
}

impl AsRef<ChatListArchive> for RTDChatListArchiveBuilder {
  fn as_ref(&self) -> &ChatListArchive {
    &self.inner
  }
}

/// A list of chats added to a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListFolder {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat folder identifier
  chat_folder_id: i64,
}

impl RObject for ChatListFolder {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatListFolder"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatList for ChatListFolder {}

impl ChatListFolder {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatListFolderBuilder {
    let inner = ChatListFolder {
      td_name: "chatListFolder".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatListFolderBuilder { inner }
  }

  pub fn chat_folder_id(&self) -> i64 {
    self.chat_folder_id
  }
}

#[doc(hidden)]
pub struct RTDChatListFolderBuilder {
  inner: ChatListFolder,
}

impl RTDChatListFolderBuilder {
  pub fn build(&self) -> ChatListFolder {
    self.inner.clone()
  }

  pub fn chat_folder_id(&mut self, chat_folder_id: i64) -> &mut Self {
    self.inner.chat_folder_id = chat_folder_id;
    self
  }
}

impl AsRef<ChatListFolder> for ChatListFolder {
  fn as_ref(&self) -> &ChatListFolder {
    self
  }
}

impl AsRef<ChatListFolder> for RTDChatListFolderBuilder {
  fn as_ref(&self) -> &ChatListFolder {
    &self.inner
  }
}

/// A main list of chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListMain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ChatListMain {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatListMain"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatList for ChatListMain {}

impl ChatListMain {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatListMainBuilder {
    let inner = ChatListMain {
      td_name: "chatListMain".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDChatListMainBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDChatListMainBuilder {
  inner: ChatListMain,
}

impl RTDChatListMainBuilder {
  pub fn build(&self) -> ChatListMain {
    self.inner.clone()
  }
}

impl AsRef<ChatListMain> for ChatListMain {
  fn as_ref(&self) -> &ChatListMain {
    self
  }
}

impl AsRef<ChatListMain> for RTDChatListMainBuilder {
  fn as_ref(&self) -> &ChatListMain {
    &self.inner
  }
}
