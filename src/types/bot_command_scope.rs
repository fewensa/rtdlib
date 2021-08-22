
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




use std::fmt::Debug;
use serde::de::{Deserialize, Deserializer};



/// TRAIT | Represents the scope to which bot commands are relevant
pub trait TDBotCommandScope: Debug + RObject {}

/// Represents the scope to which bot commands are relevant
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BotCommandScope {
  #[doc(hidden)] _Default(()),
  /// A scope covering all group and supergroup chat administrators
  AllChatAdministrators(BotCommandScopeAllChatAdministrators),
  /// A scope covering all group and supergroup chats
  AllGroupChats(BotCommandScopeAllGroupChats),
  /// A scope covering all private chats
  AllPrivateChats(BotCommandScopeAllPrivateChats),
  /// A scope covering all members of a chat
  Chat(BotCommandScopeChat),
  /// A scope covering all administrators of a chat
  ChatAdministrators(BotCommandScopeChatAdministrators),
  /// A scope covering a member of a chat
  ChatMember(BotCommandScopeChatMember),
  /// A scope covering all users
  Default(BotCommandScopeDefault),

}

impl Default for BotCommandScope {
  fn default() -> Self { BotCommandScope::_Default(()) }
}

impl<'de> Deserialize<'de> for BotCommandScope {
  fn deserialize<D>(deserializer: D) -> Result<BotCommandScope, D::Error> where D: Deserializer<'de> {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BotCommandScope,
      (botCommandScopeAllChatAdministrators, AllChatAdministrators);
      (botCommandScopeAllGroupChats, AllGroupChats);
      (botCommandScopeAllPrivateChats, AllPrivateChats);
      (botCommandScopeChat, Chat);
      (botCommandScopeChatAdministrators, ChatAdministrators);
      (botCommandScopeChatMember, ChatMember);
      (botCommandScopeDefault, Default);

    )(deserializer)
  }
}

impl RObject for BotCommandScope {
  #[doc(hidden)] fn td_name(&self) -> &'static str {
    match self {
      BotCommandScope::AllChatAdministrators(t) => t.td_name(),
      BotCommandScope::AllGroupChats(t) => t.td_name(),
      BotCommandScope::AllPrivateChats(t) => t.td_name(),
      BotCommandScope::Chat(t) => t.td_name(),
      BotCommandScope::ChatAdministrators(t) => t.td_name(),
      BotCommandScope::ChatMember(t) => t.td_name(),
      BotCommandScope::Default(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)] fn extra(&self) -> Option<String> {
    match self {
      BotCommandScope::AllChatAdministrators(t) => t.extra(),
      BotCommandScope::AllGroupChats(t) => t.extra(),
      BotCommandScope::AllPrivateChats(t) => t.extra(),
      BotCommandScope::Chat(t) => t.extra(),
      BotCommandScope::ChatAdministrators(t) => t.extra(),
      BotCommandScope::ChatMember(t) => t.extra(),
      BotCommandScope::Default(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}

impl BotCommandScope {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  #[doc(hidden)] pub fn _is_default(&self) -> bool { if let BotCommandScope::_Default(_) = self { true } else { false } }

  pub fn is_all_chat_administrators(&self) -> bool { if let BotCommandScope::AllChatAdministrators(_) = self { true } else { false } }
  pub fn is_all_group_chats(&self) -> bool { if let BotCommandScope::AllGroupChats(_) = self { true } else { false } }
  pub fn is_all_private_chats(&self) -> bool { if let BotCommandScope::AllPrivateChats(_) = self { true } else { false } }
  pub fn is_chat(&self) -> bool { if let BotCommandScope::Chat(_) = self { true } else { false } }
  pub fn is_chat_administrators(&self) -> bool { if let BotCommandScope::ChatAdministrators(_) = self { true } else { false } }
  pub fn is_chat_member(&self) -> bool { if let BotCommandScope::ChatMember(_) = self { true } else { false } }
  pub fn is_default(&self) -> bool { if let BotCommandScope::Default(_) = self { true } else { false } }

  pub fn on_all_chat_administrators<F: FnOnce(&BotCommandScopeAllChatAdministrators)>(&self, fnc: F) -> &Self { if let BotCommandScope::AllChatAdministrators(t) = self { fnc(t) }; self }
  pub fn on_all_group_chats<F: FnOnce(&BotCommandScopeAllGroupChats)>(&self, fnc: F) -> &Self { if let BotCommandScope::AllGroupChats(t) = self { fnc(t) }; self }
  pub fn on_all_private_chats<F: FnOnce(&BotCommandScopeAllPrivateChats)>(&self, fnc: F) -> &Self { if let BotCommandScope::AllPrivateChats(t) = self { fnc(t) }; self }
  pub fn on_chat<F: FnOnce(&BotCommandScopeChat)>(&self, fnc: F) -> &Self { if let BotCommandScope::Chat(t) = self { fnc(t) }; self }
  pub fn on_chat_administrators<F: FnOnce(&BotCommandScopeChatAdministrators)>(&self, fnc: F) -> &Self { if let BotCommandScope::ChatAdministrators(t) = self { fnc(t) }; self }
  pub fn on_chat_member<F: FnOnce(&BotCommandScopeChatMember)>(&self, fnc: F) -> &Self { if let BotCommandScope::ChatMember(t) = self { fnc(t) }; self }
  pub fn on_default<F: FnOnce(&BotCommandScopeDefault)>(&self, fnc: F) -> &Self { if let BotCommandScope::Default(t) = self { fnc(t) }; self }

  pub fn as_all_chat_administrators(&self) -> Option<&BotCommandScopeAllChatAdministrators> { if let BotCommandScope::AllChatAdministrators(t) = self { return Some(t) } None }
  pub fn as_all_group_chats(&self) -> Option<&BotCommandScopeAllGroupChats> { if let BotCommandScope::AllGroupChats(t) = self { return Some(t) } None }
  pub fn as_all_private_chats(&self) -> Option<&BotCommandScopeAllPrivateChats> { if let BotCommandScope::AllPrivateChats(t) = self { return Some(t) } None }
  pub fn as_chat(&self) -> Option<&BotCommandScopeChat> { if let BotCommandScope::Chat(t) = self { return Some(t) } None }
  pub fn as_chat_administrators(&self) -> Option<&BotCommandScopeChatAdministrators> { if let BotCommandScope::ChatAdministrators(t) = self { return Some(t) } None }
  pub fn as_chat_member(&self) -> Option<&BotCommandScopeChatMember> { if let BotCommandScope::ChatMember(t) = self { return Some(t) } None }
  pub fn as_default(&self) -> Option<&BotCommandScopeDefault> { if let BotCommandScope::Default(t) = self { return Some(t) } None }



  pub fn all_chat_administrators<T: AsRef<BotCommandScopeAllChatAdministrators>>(t: T) -> Self { BotCommandScope::AllChatAdministrators(t.as_ref().clone()) }

  pub fn all_group_chats<T: AsRef<BotCommandScopeAllGroupChats>>(t: T) -> Self { BotCommandScope::AllGroupChats(t.as_ref().clone()) }

  pub fn all_private_chats<T: AsRef<BotCommandScopeAllPrivateChats>>(t: T) -> Self { BotCommandScope::AllPrivateChats(t.as_ref().clone()) }

  pub fn chat<T: AsRef<BotCommandScopeChat>>(t: T) -> Self { BotCommandScope::Chat(t.as_ref().clone()) }

  pub fn chat_administrators<T: AsRef<BotCommandScopeChatAdministrators>>(t: T) -> Self { BotCommandScope::ChatAdministrators(t.as_ref().clone()) }

  pub fn chat_member<T: AsRef<BotCommandScopeChatMember>>(t: T) -> Self { BotCommandScope::ChatMember(t.as_ref().clone()) }

  pub fn default<T: AsRef<BotCommandScopeDefault>>(t: T) -> Self { BotCommandScope::Default(t.as_ref().clone()) }

}

impl AsRef<BotCommandScope> for BotCommandScope {
  fn as_ref(&self) -> &BotCommandScope { self }
}







/// A scope covering all group and supergroup chat administrators
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeAllChatAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for BotCommandScopeAllChatAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommandScopeAllChatAdministrators" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBotCommandScope for BotCommandScopeAllChatAdministrators {}



impl BotCommandScopeAllChatAdministrators {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandScopeAllChatAdministratorsBuilder {
    let mut inner = BotCommandScopeAllChatAdministrators::default();
    inner.td_name = "botCommandScopeAllChatAdministrators".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandScopeAllChatAdministratorsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDBotCommandScopeAllChatAdministratorsBuilder {
  inner: BotCommandScopeAllChatAdministrators
}

impl RTDBotCommandScopeAllChatAdministratorsBuilder {
  pub fn build(&self) -> BotCommandScopeAllChatAdministrators { self.inner.clone() }

}

impl AsRef<BotCommandScopeAllChatAdministrators> for BotCommandScopeAllChatAdministrators {
  fn as_ref(&self) -> &BotCommandScopeAllChatAdministrators { self }
}

impl AsRef<BotCommandScopeAllChatAdministrators> for RTDBotCommandScopeAllChatAdministratorsBuilder {
  fn as_ref(&self) -> &BotCommandScopeAllChatAdministrators { &self.inner }
}







/// A scope covering all group and supergroup chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeAllGroupChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for BotCommandScopeAllGroupChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommandScopeAllGroupChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBotCommandScope for BotCommandScopeAllGroupChats {}



impl BotCommandScopeAllGroupChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandScopeAllGroupChatsBuilder {
    let mut inner = BotCommandScopeAllGroupChats::default();
    inner.td_name = "botCommandScopeAllGroupChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandScopeAllGroupChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDBotCommandScopeAllGroupChatsBuilder {
  inner: BotCommandScopeAllGroupChats
}

impl RTDBotCommandScopeAllGroupChatsBuilder {
  pub fn build(&self) -> BotCommandScopeAllGroupChats { self.inner.clone() }

}

impl AsRef<BotCommandScopeAllGroupChats> for BotCommandScopeAllGroupChats {
  fn as_ref(&self) -> &BotCommandScopeAllGroupChats { self }
}

impl AsRef<BotCommandScopeAllGroupChats> for RTDBotCommandScopeAllGroupChatsBuilder {
  fn as_ref(&self) -> &BotCommandScopeAllGroupChats { &self.inner }
}







/// A scope covering all private chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeAllPrivateChats {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for BotCommandScopeAllPrivateChats {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommandScopeAllPrivateChats" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBotCommandScope for BotCommandScopeAllPrivateChats {}



impl BotCommandScopeAllPrivateChats {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandScopeAllPrivateChatsBuilder {
    let mut inner = BotCommandScopeAllPrivateChats::default();
    inner.td_name = "botCommandScopeAllPrivateChats".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandScopeAllPrivateChatsBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDBotCommandScopeAllPrivateChatsBuilder {
  inner: BotCommandScopeAllPrivateChats
}

impl RTDBotCommandScopeAllPrivateChatsBuilder {
  pub fn build(&self) -> BotCommandScopeAllPrivateChats { self.inner.clone() }

}

impl AsRef<BotCommandScopeAllPrivateChats> for BotCommandScopeAllPrivateChats {
  fn as_ref(&self) -> &BotCommandScopeAllPrivateChats { self }
}

impl AsRef<BotCommandScopeAllPrivateChats> for RTDBotCommandScopeAllPrivateChatsBuilder {
  fn as_ref(&self) -> &BotCommandScopeAllPrivateChats { &self.inner }
}







/// A scope covering all members of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for BotCommandScopeChat {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommandScopeChat" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBotCommandScope for BotCommandScopeChat {}



impl BotCommandScopeChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandScopeChatBuilder {
    let mut inner = BotCommandScopeChat::default();
    inner.td_name = "botCommandScopeChat".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandScopeChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDBotCommandScopeChatBuilder {
  inner: BotCommandScopeChat
}

impl RTDBotCommandScopeChatBuilder {
  pub fn build(&self) -> BotCommandScopeChat { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<BotCommandScopeChat> for BotCommandScopeChat {
  fn as_ref(&self) -> &BotCommandScopeChat { self }
}

impl AsRef<BotCommandScopeChat> for RTDBotCommandScopeChatBuilder {
  fn as_ref(&self) -> &BotCommandScopeChat { &self.inner }
}







/// A scope covering all administrators of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeChatAdministrators {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  
}

impl RObject for BotCommandScopeChatAdministrators {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommandScopeChatAdministrators" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBotCommandScope for BotCommandScopeChatAdministrators {}



impl BotCommandScopeChatAdministrators {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandScopeChatAdministratorsBuilder {
    let mut inner = BotCommandScopeChatAdministrators::default();
    inner.td_name = "botCommandScopeChatAdministrators".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandScopeChatAdministratorsBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

}

#[doc(hidden)]
pub struct RTDBotCommandScopeChatAdministratorsBuilder {
  inner: BotCommandScopeChatAdministrators
}

impl RTDBotCommandScopeChatAdministratorsBuilder {
  pub fn build(&self) -> BotCommandScopeChatAdministrators { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

}

impl AsRef<BotCommandScopeChatAdministrators> for BotCommandScopeChatAdministrators {
  fn as_ref(&self) -> &BotCommandScopeChatAdministrators { self }
}

impl AsRef<BotCommandScopeChatAdministrators> for RTDBotCommandScopeChatAdministratorsBuilder {
  fn as_ref(&self) -> &BotCommandScopeChatAdministrators { &self.inner }
}







/// A scope covering a member of a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeChatMember {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Chat identifier
  chat_id: i64,
  /// User identifier
  user_id: i64,
  
}

impl RObject for BotCommandScopeChatMember {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommandScopeChatMember" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBotCommandScope for BotCommandScopeChatMember {}



impl BotCommandScopeChatMember {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandScopeChatMemberBuilder {
    let mut inner = BotCommandScopeChatMember::default();
    inner.td_name = "botCommandScopeChatMember".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandScopeChatMemberBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 { self.chat_id }

  pub fn user_id(&self) -> i64 { self.user_id }

}

#[doc(hidden)]
pub struct RTDBotCommandScopeChatMemberBuilder {
  inner: BotCommandScopeChatMember
}

impl RTDBotCommandScopeChatMemberBuilder {
  pub fn build(&self) -> BotCommandScopeChatMember { self.inner.clone() }

   
  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }

   
  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

}

impl AsRef<BotCommandScopeChatMember> for BotCommandScopeChatMember {
  fn as_ref(&self) -> &BotCommandScopeChatMember { self }
}

impl AsRef<BotCommandScopeChatMember> for RTDBotCommandScopeChatMemberBuilder {
  fn as_ref(&self) -> &BotCommandScopeChatMember { &self.inner }
}







/// A scope covering all users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotCommandScopeDefault {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  
}

impl RObject for BotCommandScopeDefault {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "botCommandScopeDefault" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}


impl TDBotCommandScope for BotCommandScopeDefault {}



impl BotCommandScopeDefault {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDBotCommandScopeDefaultBuilder {
    let mut inner = BotCommandScopeDefault::default();
    inner.td_name = "botCommandScopeDefault".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDBotCommandScopeDefaultBuilder { inner }
  }

}

#[doc(hidden)]
pub struct RTDBotCommandScopeDefaultBuilder {
  inner: BotCommandScopeDefault
}

impl RTDBotCommandScopeDefaultBuilder {
  pub fn build(&self) -> BotCommandScopeDefault { self.inner.clone() }

}

impl AsRef<BotCommandScopeDefault> for BotCommandScopeDefault {
  fn as_ref(&self) -> &BotCommandScopeDefault { self }
}

impl AsRef<BotCommandScopeDefault> for RTDBotCommandScopeDefaultBuilder {
  fn as_ref(&self) -> &BotCommandScopeDefault { &self.inner }
}



