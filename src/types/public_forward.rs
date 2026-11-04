use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a public forward or repost of a story
pub trait TDPublicForward: Debug + RObject {}

/// Describes a public forward or repost of a story
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PublicForward {
  #[doc(hidden)]
  _Default(()),
  /// Contains a public forward as a message
  Message(PublicForwardMessage),
  /// Contains a public repost to a story
  Story(PublicForwardStory),
}

impl Default for PublicForward {
  fn default() -> Self {
    PublicForward::_Default(())
  }
}

impl<'de> Deserialize<'de> for PublicForward {
  fn deserialize<D>(deserializer: D) -> Result<PublicForward, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PublicForward,
      (publicForwardMessage, Message);
      (publicForwardStory, Story);

    )(deserializer)
  }
}

impl RObject for PublicForward {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PublicForward::Message(t) => t.td_name(),
      PublicForward::Story(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PublicForward::Message(t) => t.extra(),
      PublicForward::Story(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PublicForward {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PublicForward::_Default(_))
  }

  pub fn is_message(&self) -> bool {
    matches!(self, PublicForward::Message(_))
  }
  pub fn is_story(&self) -> bool {
    matches!(self, PublicForward::Story(_))
  }

  pub fn on_message<F: FnOnce(&PublicForwardMessage)>(&self, fnc: F) -> &Self {
    if let PublicForward::Message(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story<F: FnOnce(&PublicForwardStory)>(&self, fnc: F) -> &Self {
    if let PublicForward::Story(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_message(&self) -> Option<&PublicForwardMessage> {
    if let PublicForward::Message(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story(&self) -> Option<&PublicForwardStory> {
    if let PublicForward::Story(t) = self {
      return Some(t);
    }
    None
  }

  pub fn message<T: AsRef<PublicForwardMessage>>(t: T) -> Self {
    PublicForward::Message(t.as_ref().clone())
  }

  pub fn story<T: AsRef<PublicForwardStory>>(t: T) -> Self {
    PublicForward::Story(t.as_ref().clone())
  }
}

impl AsRef<PublicForward> for PublicForward {
  fn as_ref(&self) -> &PublicForward {
    self
  }
}

/// Contains a public forward as a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicForwardMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the message
  message: Message,
}

impl RObject for PublicForwardMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "publicForwardMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPublicForward for PublicForwardMessage {}

impl PublicForwardMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPublicForwardMessageBuilder {
    let inner = PublicForwardMessage {
      td_name: "publicForwardMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPublicForwardMessageBuilder { inner }
  }

  pub fn message(&self) -> &Message {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDPublicForwardMessageBuilder {
  inner: PublicForwardMessage,
}

impl RTDPublicForwardMessageBuilder {
  pub fn build(&self) -> PublicForwardMessage {
    self.inner.clone()
  }

  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }
}

impl AsRef<PublicForwardMessage> for PublicForwardMessage {
  fn as_ref(&self) -> &PublicForwardMessage {
    self
  }
}

impl AsRef<PublicForwardMessage> for RTDPublicForwardMessageBuilder {
  fn as_ref(&self) -> &PublicForwardMessage {
    &self.inner
  }
}

/// Contains a public repost to a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PublicForwardStory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the story
  story: Story,
}

impl RObject for PublicForwardStory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "publicForwardStory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPublicForward for PublicForwardStory {}

impl PublicForwardStory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPublicForwardStoryBuilder {
    let inner = PublicForwardStory {
      td_name: "publicForwardStory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPublicForwardStoryBuilder { inner }
  }

  pub fn story(&self) -> &Story {
    &self.story
  }
}

#[doc(hidden)]
pub struct RTDPublicForwardStoryBuilder {
  inner: PublicForwardStory,
}

impl RTDPublicForwardStoryBuilder {
  pub fn build(&self) -> PublicForwardStory {
    self.inner.clone()
  }

  pub fn story<T: AsRef<Story>>(&mut self, story: T) -> &mut Self {
    self.inner.story = story.as_ref().clone();
    self
  }
}

impl AsRef<PublicForwardStory> for PublicForwardStory {
  fn as_ref(&self) -> &PublicForwardStory {
    self
  }
}

impl AsRef<PublicForwardStory> for RTDPublicForwardStoryBuilder {
  fn as_ref(&self) -> &PublicForwardStory {
    &self.inner
  }
}
