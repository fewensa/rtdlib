use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes reactions available in the chat
pub trait TDChatAvailableReactions: Debug + RObject {}

/// Describes reactions available in the chat
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ChatAvailableReactions {
  #[doc(hidden)]
  _Default(()),
  /// All reactions are available in the chat, excluding the paid reaction and custom reactions in channel chats
  All(ChatAvailableReactionsAll),
  /// Only specific reactions are available in the chat
  Some(ChatAvailableReactionsSome),
}

impl Default for ChatAvailableReactions {
  fn default() -> Self {
    ChatAvailableReactions::_Default(())
  }
}

impl<'de> Deserialize<'de> for ChatAvailableReactions {
  fn deserialize<D>(deserializer: D) -> Result<ChatAvailableReactions, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ChatAvailableReactions,
      (chatAvailableReactionsAll, All);
      (chatAvailableReactionsSome, Some);

    )(deserializer)
  }
}

impl RObject for ChatAvailableReactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ChatAvailableReactions::All(t) => t.td_name(),
      ChatAvailableReactions::Some(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ChatAvailableReactions::All(t) => t.extra(),
      ChatAvailableReactions::Some(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatAvailableReactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ChatAvailableReactions::_Default(_))
  }

  pub fn is_all(&self) -> bool {
    matches!(self, ChatAvailableReactions::All(_))
  }
  pub fn is_some(&self) -> bool {
    matches!(self, ChatAvailableReactions::Some(_))
  }

  pub fn on_all<F: FnOnce(&ChatAvailableReactionsAll)>(&self, fnc: F) -> &Self {
    if let ChatAvailableReactions::All(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_some<F: FnOnce(&ChatAvailableReactionsSome)>(&self, fnc: F) -> &Self {
    if let ChatAvailableReactions::Some(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_all(&self) -> Option<&ChatAvailableReactionsAll> {
    if let ChatAvailableReactions::All(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_some(&self) -> Option<&ChatAvailableReactionsSome> {
    if let ChatAvailableReactions::Some(t) = self {
      return Some(t);
    }
    None
  }

  pub fn all<T: AsRef<ChatAvailableReactionsAll>>(t: T) -> Self {
    ChatAvailableReactions::All(t.as_ref().clone())
  }

  pub fn some<T: AsRef<ChatAvailableReactionsSome>>(t: T) -> Self {
    ChatAvailableReactions::Some(t.as_ref().clone())
  }
}

impl AsRef<ChatAvailableReactions> for ChatAvailableReactions {
  fn as_ref(&self) -> &ChatAvailableReactions {
    self
  }
}

/// All reactions are available in the chat, excluding the paid reaction and custom reactions in channel chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAvailableReactionsAll {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The maximum allowed number of reactions per message; 1-11
  max_reaction_count: i64,
}

impl RObject for ChatAvailableReactionsAll {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatAvailableReactionsAll"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatAvailableReactions for ChatAvailableReactionsAll {}

impl ChatAvailableReactionsAll {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatAvailableReactionsAllBuilder {
    let inner = ChatAvailableReactionsAll {
      td_name: "chatAvailableReactionsAll".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatAvailableReactionsAllBuilder { inner }
  }

  pub fn max_reaction_count(&self) -> i64 {
    self.max_reaction_count
  }
}

#[doc(hidden)]
pub struct RTDChatAvailableReactionsAllBuilder {
  inner: ChatAvailableReactionsAll,
}

impl RTDChatAvailableReactionsAllBuilder {
  pub fn build(&self) -> ChatAvailableReactionsAll {
    self.inner.clone()
  }

  pub fn max_reaction_count(&mut self, max_reaction_count: i64) -> &mut Self {
    self.inner.max_reaction_count = max_reaction_count;
    self
  }
}

impl AsRef<ChatAvailableReactionsAll> for ChatAvailableReactionsAll {
  fn as_ref(&self) -> &ChatAvailableReactionsAll {
    self
  }
}

impl AsRef<ChatAvailableReactionsAll> for RTDChatAvailableReactionsAllBuilder {
  fn as_ref(&self) -> &ChatAvailableReactionsAll {
    &self.inner
  }
}

/// Only specific reactions are available in the chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatAvailableReactionsSome {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of reactions
  reactions: Vec<ReactionType>,
  /// The maximum allowed number of reactions per message; 1-11
  max_reaction_count: i64,
}

impl RObject for ChatAvailableReactionsSome {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatAvailableReactionsSome"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDChatAvailableReactions for ChatAvailableReactionsSome {}

impl ChatAvailableReactionsSome {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatAvailableReactionsSomeBuilder {
    let inner = ChatAvailableReactionsSome {
      td_name: "chatAvailableReactionsSome".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatAvailableReactionsSomeBuilder { inner }
  }

  pub fn reactions(&self) -> &Vec<ReactionType> {
    &self.reactions
  }

  pub fn max_reaction_count(&self) -> i64 {
    self.max_reaction_count
  }
}

#[doc(hidden)]
pub struct RTDChatAvailableReactionsSomeBuilder {
  inner: ChatAvailableReactionsSome,
}

impl RTDChatAvailableReactionsSomeBuilder {
  pub fn build(&self) -> ChatAvailableReactionsSome {
    self.inner.clone()
  }

  pub fn reactions(&mut self, reactions: Vec<ReactionType>) -> &mut Self {
    self.inner.reactions = reactions;
    self
  }

  pub fn max_reaction_count(&mut self, max_reaction_count: i64) -> &mut Self {
    self.inner.max_reaction_count = max_reaction_count;
    self
  }
}

impl AsRef<ChatAvailableReactionsSome> for ChatAvailableReactionsSome {
  fn as_ref(&self) -> &ChatAvailableReactionsSome {
    self
  }
}

impl AsRef<ChatAvailableReactionsSome> for RTDChatAvailableReactionsSomeBuilder {
  fn as_ref(&self) -> &ChatAvailableReactionsSome {
    &self.inner
  }
}
