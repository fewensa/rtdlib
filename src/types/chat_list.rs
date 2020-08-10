
use crate::types::*;
use crate::errors::*;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Describes a list of chats
pub trait TDChatList: Debug + RObject {}

/// Describes a list of chats
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatList {
  #[doc(hidden)] _Default(()),
  /// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
  Archive(ChatListArchive),
  /// A list of chats belonging to a chat filter
  Filter(ChatListFilter),
  /// A main list of chats
  Main(ChatListMain),

}

impl Default for ChatList {
  fn default() -> Self { ChatList::_Default(()) }
}

impl<'de> Deserialize<'de> for ChatList {
  fn deserialize<D>(deserializer: D) -> Result<ChatList, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatList,
      (chatListArchive, Archive);
      (chatListFilter, Filter);
      (chatListMain, Main);

    )(deserializer)
  }
}

impl RObject for ChatList {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      ChatList::Archive(t) => t.td_name(),
      ChatList::Filter(t) => t.td_name(),
      ChatList::Main(t) => t.td_name(),

      _ => "-1",
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl ChatList {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let ChatList::_Default(_) = self { true } else { false } }

  pub fn is_archive(&self) -> bool { if let ChatList::Archive(_) = self { true } else { false } }
  pub fn is_filter(&self) -> bool { if let ChatList::Filter(_) = self { true } else { false } }
  pub fn is_main(&self) -> bool { if let ChatList::Main(_) = self { true } else { false } }

  pub fn on_archive<F: FnOnce(&ChatListArchive)>(&self, fnc: F) -> &Self { if let ChatList::Archive(t) = self { fnc(t) }; self }
  pub fn on_filter<F: FnOnce(&ChatListFilter)>(&self, fnc: F) -> &Self { if let ChatList::Filter(t) = self { fnc(t) }; self }
  pub fn on_main<F: FnOnce(&ChatListMain)>(&self, fnc: F) -> &Self { if let ChatList::Main(t) = self { fnc(t) }; self }

  pub fn as_archive(&self) -> Option<&ChatListArchive> { if let ChatList::Archive(t) = self { return Some(t) } None }
  pub fn as_filter(&self) -> Option<&ChatListFilter> { if let ChatList::Filter(t) = self { return Some(t) } None }
  pub fn as_main(&self) -> Option<&ChatListMain> { if let ChatList::Main(t) = self { return Some(t) } None }



  pub fn archive<T: AsRef<ChatListArchive>>(t: T) -> Self { ChatList::Archive(t.as_ref().clone()) }

  pub fn filter<T: AsRef<ChatListFilter>>(t: T) -> Self { ChatList::Filter(t.as_ref().clone()) }

  pub fn main<T: AsRef<ChatListMain>>(t: T) -> Self { ChatList::Main(t.as_ref().clone()) }

}

impl AsRef<ChatList> for ChatList {
  fn as_ref(&self) -> &ChatList { self }
}







/// A list of chats usually located at the top of the main chat list. Unmuted chats are automatically moved from the Archive to the Main chat list when a new message arrives
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListArchive {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatListArchive {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatListArchive" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatList for ChatListArchive {}



impl ChatListArchive {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatListArchiveBuilder {
    let mut inner = ChatListArchive::default();
    inner.td_name = "chatListArchive".to_string();
    RTDChatListArchiveBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatListArchiveBuilder {
  inner: ChatListArchive
}

impl RTDChatListArchiveBuilder {
  pub fn build(&self) -> ChatListArchive { self.inner.clone() }

}

impl AsRef<ChatListArchive> for ChatListArchive {
  fn as_ref(&self) -> &ChatListArchive { self }
}

impl AsRef<ChatListArchive> for RTDChatListArchiveBuilder {
  fn as_ref(&self) -> &ChatListArchive { &self.inner }
}







/// A list of chats belonging to a chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListFilter {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Chat filter identifier
  chat_filter_id: i64,
  
}

impl RObject for ChatListFilter {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatListFilter" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatList for ChatListFilter {}



impl ChatListFilter {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatListFilterBuilder {
    let mut inner = ChatListFilter::default();
    inner.td_name = "chatListFilter".to_string();
    RTDChatListFilterBuilder { inner }
  }

  pub fn chat_filter_id(&self) -> i64 { self.chat_filter_id }

}

#[doc(hidden)]
pub struct RTDChatListFilterBuilder {
  inner: ChatListFilter
}

impl RTDChatListFilterBuilder {
  pub fn build(&self) -> ChatListFilter { self.inner.clone() }

   
  pub fn chat_filter_id(&mut self, chat_filter_id: i64) -> &mut Self {
    self.inner.chat_filter_id = chat_filter_id;
    self
  }

}

impl AsRef<ChatListFilter> for ChatListFilter {
  fn as_ref(&self) -> &ChatListFilter { self }
}

impl AsRef<ChatListFilter> for RTDChatListFilterBuilder {
  fn as_ref(&self) -> &ChatListFilter { &self.inner }
}







/// A main list of chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatListMain {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  
}

impl RObject for ChatListMain {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatListMain" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDChatList for ChatListMain {}



impl ChatListMain {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatListMainBuilder {
    let mut inner = ChatListMain::default();
    inner.td_name = "chatListMain".to_string();
    RTDChatListMainBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDChatListMainBuilder {
  inner: ChatListMain
}

impl RTDChatListMainBuilder {
  pub fn build(&self) -> ChatListMain { self.inner.clone() }

}

impl AsRef<ChatListMain> for ChatListMain {
  fn as_ref(&self) -> &ChatListMain { self }
}

impl AsRef<ChatListMain> for RTDChatListMainBuilder {
  fn as_ref(&self) -> &ChatListMain { &self.inner }
}



