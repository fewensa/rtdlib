use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of Saved Messages topic
pub trait TDSavedMessagesTopicType: Debug + RObject {}

/// Describes type of Saved Messages topic
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SavedMessagesTopicType {
  #[doc(hidden)]
  _Default(()),
  /// Topic containing messages forwarded from a user with hidden privacy
  AuthorHidden(SavedMessagesTopicTypeAuthorHidden),
  /// Topic containing messages sent by the current user or forwarded from an unknown chat
  MyNotes(SavedMessagesTopicTypeMyNotes),
  /// Topic containing messages forwarded from a specific chat
  SavedFromChat(SavedMessagesTopicTypeSavedFromChat),
}

impl Default for SavedMessagesTopicType {
  fn default() -> Self {
    SavedMessagesTopicType::_Default(())
  }
}

impl<'de> Deserialize<'de> for SavedMessagesTopicType {
  fn deserialize<D>(deserializer: D) -> Result<SavedMessagesTopicType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SavedMessagesTopicType,
      (savedMessagesTopicTypeAuthorHidden, AuthorHidden);
      (savedMessagesTopicTypeMyNotes, MyNotes);
      (savedMessagesTopicTypeSavedFromChat, SavedFromChat);

    )(deserializer)
  }
}

impl RObject for SavedMessagesTopicType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SavedMessagesTopicType::AuthorHidden(t) => t.td_name(),
      SavedMessagesTopicType::MyNotes(t) => t.td_name(),
      SavedMessagesTopicType::SavedFromChat(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SavedMessagesTopicType::AuthorHidden(t) => t.extra(),
      SavedMessagesTopicType::MyNotes(t) => t.extra(),
      SavedMessagesTopicType::SavedFromChat(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SavedMessagesTopicType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SavedMessagesTopicType::_Default(_))
  }

  pub fn is_author_hidden(&self) -> bool {
    matches!(self, SavedMessagesTopicType::AuthorHidden(_))
  }
  pub fn is_my_notes(&self) -> bool {
    matches!(self, SavedMessagesTopicType::MyNotes(_))
  }
  pub fn is_saved_from_chat(&self) -> bool {
    matches!(self, SavedMessagesTopicType::SavedFromChat(_))
  }

  pub fn on_author_hidden<F: FnOnce(&SavedMessagesTopicTypeAuthorHidden)>(&self, fnc: F) -> &Self {
    if let SavedMessagesTopicType::AuthorHidden(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_my_notes<F: FnOnce(&SavedMessagesTopicTypeMyNotes)>(&self, fnc: F) -> &Self {
    if let SavedMessagesTopicType::MyNotes(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_saved_from_chat<F: FnOnce(&SavedMessagesTopicTypeSavedFromChat)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let SavedMessagesTopicType::SavedFromChat(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_author_hidden(&self) -> Option<&SavedMessagesTopicTypeAuthorHidden> {
    if let SavedMessagesTopicType::AuthorHidden(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_my_notes(&self) -> Option<&SavedMessagesTopicTypeMyNotes> {
    if let SavedMessagesTopicType::MyNotes(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_saved_from_chat(&self) -> Option<&SavedMessagesTopicTypeSavedFromChat> {
    if let SavedMessagesTopicType::SavedFromChat(t) = self {
      return Some(t);
    }
    None
  }

  pub fn author_hidden<T: AsRef<SavedMessagesTopicTypeAuthorHidden>>(t: T) -> Self {
    SavedMessagesTopicType::AuthorHidden(t.as_ref().clone())
  }

  pub fn my_notes<T: AsRef<SavedMessagesTopicTypeMyNotes>>(t: T) -> Self {
    SavedMessagesTopicType::MyNotes(t.as_ref().clone())
  }

  pub fn saved_from_chat<T: AsRef<SavedMessagesTopicTypeSavedFromChat>>(t: T) -> Self {
    SavedMessagesTopicType::SavedFromChat(t.as_ref().clone())
  }
}

impl AsRef<SavedMessagesTopicType> for SavedMessagesTopicType {
  fn as_ref(&self) -> &SavedMessagesTopicType {
    self
  }
}

/// Topic containing messages forwarded from a user with hidden privacy
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedMessagesTopicTypeAuthorHidden {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SavedMessagesTopicTypeAuthorHidden {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "savedMessagesTopicTypeAuthorHidden"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSavedMessagesTopicType for SavedMessagesTopicTypeAuthorHidden {}

impl SavedMessagesTopicTypeAuthorHidden {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSavedMessagesTopicTypeAuthorHiddenBuilder {
    let inner = SavedMessagesTopicTypeAuthorHidden {
      td_name: "savedMessagesTopicTypeAuthorHidden".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSavedMessagesTopicTypeAuthorHiddenBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSavedMessagesTopicTypeAuthorHiddenBuilder {
  inner: SavedMessagesTopicTypeAuthorHidden,
}

impl RTDSavedMessagesTopicTypeAuthorHiddenBuilder {
  pub fn build(&self) -> SavedMessagesTopicTypeAuthorHidden {
    self.inner.clone()
  }
}

impl AsRef<SavedMessagesTopicTypeAuthorHidden> for SavedMessagesTopicTypeAuthorHidden {
  fn as_ref(&self) -> &SavedMessagesTopicTypeAuthorHidden {
    self
  }
}

impl AsRef<SavedMessagesTopicTypeAuthorHidden> for RTDSavedMessagesTopicTypeAuthorHiddenBuilder {
  fn as_ref(&self) -> &SavedMessagesTopicTypeAuthorHidden {
    &self.inner
  }
}

/// Topic containing messages sent by the current user or forwarded from an unknown chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedMessagesTopicTypeMyNotes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for SavedMessagesTopicTypeMyNotes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "savedMessagesTopicTypeMyNotes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSavedMessagesTopicType for SavedMessagesTopicTypeMyNotes {}

impl SavedMessagesTopicTypeMyNotes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSavedMessagesTopicTypeMyNotesBuilder {
    let inner = SavedMessagesTopicTypeMyNotes {
      td_name: "savedMessagesTopicTypeMyNotes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDSavedMessagesTopicTypeMyNotesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDSavedMessagesTopicTypeMyNotesBuilder {
  inner: SavedMessagesTopicTypeMyNotes,
}

impl RTDSavedMessagesTopicTypeMyNotesBuilder {
  pub fn build(&self) -> SavedMessagesTopicTypeMyNotes {
    self.inner.clone()
  }
}

impl AsRef<SavedMessagesTopicTypeMyNotes> for SavedMessagesTopicTypeMyNotes {
  fn as_ref(&self) -> &SavedMessagesTopicTypeMyNotes {
    self
  }
}

impl AsRef<SavedMessagesTopicTypeMyNotes> for RTDSavedMessagesTopicTypeMyNotesBuilder {
  fn as_ref(&self) -> &SavedMessagesTopicTypeMyNotes {
    &self.inner
  }
}

/// Topic containing messages forwarded from a specific chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SavedMessagesTopicTypeSavedFromChat {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the chat
  chat_id: i64,
}

impl RObject for SavedMessagesTopicTypeSavedFromChat {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "savedMessagesTopicTypeSavedFromChat"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSavedMessagesTopicType for SavedMessagesTopicTypeSavedFromChat {}

impl SavedMessagesTopicTypeSavedFromChat {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSavedMessagesTopicTypeSavedFromChatBuilder {
    let inner = SavedMessagesTopicTypeSavedFromChat {
      td_name: "savedMessagesTopicTypeSavedFromChat".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSavedMessagesTopicTypeSavedFromChatBuilder { inner }
  }

  pub fn chat_id(&self) -> i64 {
    self.chat_id
  }
}

#[doc(hidden)]
pub struct RTDSavedMessagesTopicTypeSavedFromChatBuilder {
  inner: SavedMessagesTopicTypeSavedFromChat,
}

impl RTDSavedMessagesTopicTypeSavedFromChatBuilder {
  pub fn build(&self) -> SavedMessagesTopicTypeSavedFromChat {
    self.inner.clone()
  }

  pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
    self.inner.chat_id = chat_id;
    self
  }
}

impl AsRef<SavedMessagesTopicTypeSavedFromChat> for SavedMessagesTopicTypeSavedFromChat {
  fn as_ref(&self) -> &SavedMessagesTopicTypeSavedFromChat {
    self
  }
}

impl AsRef<SavedMessagesTopicTypeSavedFromChat> for RTDSavedMessagesTopicTypeSavedFromChatBuilder {
  fn as_ref(&self) -> &SavedMessagesTopicTypeSavedFromChat {
    &self.inner
  }
}
