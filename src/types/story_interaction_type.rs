use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes type of interaction with a story
pub trait TDStoryInteractionType: Debug + RObject {}

/// Describes type of interaction with a story
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum StoryInteractionType {
  #[doc(hidden)]
  _Default(()),
  /// A forward of the story as a message
  Forward(StoryInteractionTypeForward),
  /// A repost of the story as a story
  Repost(StoryInteractionTypeRepost),
  /// A view of the story
  View(StoryInteractionTypeView),
}

impl Default for StoryInteractionType {
  fn default() -> Self {
    StoryInteractionType::_Default(())
  }
}

impl<'de> Deserialize<'de> for StoryInteractionType {
  fn deserialize<D>(deserializer: D) -> Result<StoryInteractionType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      StoryInteractionType,
      (storyInteractionTypeForward, Forward);
      (storyInteractionTypeRepost, Repost);
      (storyInteractionTypeView, View);

    )(deserializer)
  }
}

impl RObject for StoryInteractionType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      StoryInteractionType::Forward(t) => t.td_name(),
      StoryInteractionType::Repost(t) => t.td_name(),
      StoryInteractionType::View(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      StoryInteractionType::Forward(t) => t.extra(),
      StoryInteractionType::Repost(t) => t.extra(),
      StoryInteractionType::View(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryInteractionType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, StoryInteractionType::_Default(_))
  }

  pub fn is_forward(&self) -> bool {
    matches!(self, StoryInteractionType::Forward(_))
  }
  pub fn is_repost(&self) -> bool {
    matches!(self, StoryInteractionType::Repost(_))
  }
  pub fn is_view(&self) -> bool {
    matches!(self, StoryInteractionType::View(_))
  }

  pub fn on_forward<F: FnOnce(&StoryInteractionTypeForward)>(&self, fnc: F) -> &Self {
    if let StoryInteractionType::Forward(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_repost<F: FnOnce(&StoryInteractionTypeRepost)>(&self, fnc: F) -> &Self {
    if let StoryInteractionType::Repost(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_view<F: FnOnce(&StoryInteractionTypeView)>(&self, fnc: F) -> &Self {
    if let StoryInteractionType::View(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_forward(&self) -> Option<&StoryInteractionTypeForward> {
    if let StoryInteractionType::Forward(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_repost(&self) -> Option<&StoryInteractionTypeRepost> {
    if let StoryInteractionType::Repost(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_view(&self) -> Option<&StoryInteractionTypeView> {
    if let StoryInteractionType::View(t) = self {
      return Some(t);
    }
    None
  }

  pub fn forward<T: AsRef<StoryInteractionTypeForward>>(t: T) -> Self {
    StoryInteractionType::Forward(t.as_ref().clone())
  }

  pub fn repost<T: AsRef<StoryInteractionTypeRepost>>(t: T) -> Self {
    StoryInteractionType::Repost(t.as_ref().clone())
  }

  pub fn view<T: AsRef<StoryInteractionTypeView>>(t: T) -> Self {
    StoryInteractionType::View(t.as_ref().clone())
  }
}

impl AsRef<StoryInteractionType> for StoryInteractionType {
  fn as_ref(&self) -> &StoryInteractionType {
    self
  }
}

/// A forward of the story as a message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInteractionTypeForward {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The message with story forward
  message: Message,
}

impl RObject for StoryInteractionTypeForward {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyInteractionTypeForward"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryInteractionType for StoryInteractionTypeForward {}

impl StoryInteractionTypeForward {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryInteractionTypeForwardBuilder {
    let inner = StoryInteractionTypeForward {
      td_name: "storyInteractionTypeForward".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryInteractionTypeForwardBuilder { inner }
  }

  pub fn message(&self) -> &Message {
    &self.message
  }
}

#[doc(hidden)]
pub struct RTDStoryInteractionTypeForwardBuilder {
  inner: StoryInteractionTypeForward,
}

impl RTDStoryInteractionTypeForwardBuilder {
  pub fn build(&self) -> StoryInteractionTypeForward {
    self.inner.clone()
  }

  pub fn message<T: AsRef<Message>>(&mut self, message: T) -> &mut Self {
    self.inner.message = message.as_ref().clone();
    self
  }
}

impl AsRef<StoryInteractionTypeForward> for StoryInteractionTypeForward {
  fn as_ref(&self) -> &StoryInteractionTypeForward {
    self
  }
}

impl AsRef<StoryInteractionTypeForward> for RTDStoryInteractionTypeForwardBuilder {
  fn as_ref(&self) -> &StoryInteractionTypeForward {
    &self.inner
  }
}

/// A repost of the story as a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInteractionTypeRepost {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The reposted story
  story: Story,
}

impl RObject for StoryInteractionTypeRepost {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyInteractionTypeRepost"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryInteractionType for StoryInteractionTypeRepost {}

impl StoryInteractionTypeRepost {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryInteractionTypeRepostBuilder {
    let inner = StoryInteractionTypeRepost {
      td_name: "storyInteractionTypeRepost".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryInteractionTypeRepostBuilder { inner }
  }

  pub fn story(&self) -> &Story {
    &self.story
  }
}

#[doc(hidden)]
pub struct RTDStoryInteractionTypeRepostBuilder {
  inner: StoryInteractionTypeRepost,
}

impl RTDStoryInteractionTypeRepostBuilder {
  pub fn build(&self) -> StoryInteractionTypeRepost {
    self.inner.clone()
  }

  pub fn story<T: AsRef<Story>>(&mut self, story: T) -> &mut Self {
    self.inner.story = story.as_ref().clone();
    self
  }
}

impl AsRef<StoryInteractionTypeRepost> for StoryInteractionTypeRepost {
  fn as_ref(&self) -> &StoryInteractionTypeRepost {
    self
  }
}

impl AsRef<StoryInteractionTypeRepost> for RTDStoryInteractionTypeRepostBuilder {
  fn as_ref(&self) -> &StoryInteractionTypeRepost {
    &self.inner
  }
}

/// A view of the story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryInteractionTypeView {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the reaction that was chosen by the viewer; may be null if none
  chosen_reaction_type: Option<ReactionType>,
}

impl RObject for StoryInteractionTypeView {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyInteractionTypeView"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDStoryInteractionType for StoryInteractionTypeView {}

impl StoryInteractionTypeView {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryInteractionTypeViewBuilder {
    let inner = StoryInteractionTypeView {
      td_name: "storyInteractionTypeView".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryInteractionTypeViewBuilder { inner }
  }

  pub fn chosen_reaction_type(&self) -> &Option<ReactionType> {
    &self.chosen_reaction_type
  }
}

#[doc(hidden)]
pub struct RTDStoryInteractionTypeViewBuilder {
  inner: StoryInteractionTypeView,
}

impl RTDStoryInteractionTypeViewBuilder {
  pub fn build(&self) -> StoryInteractionTypeView {
    self.inner.clone()
  }

  pub fn chosen_reaction_type<T: AsRef<ReactionType>>(
    &mut self,
    chosen_reaction_type: T,
  ) -> &mut Self {
    self.inner.chosen_reaction_type = Some(chosen_reaction_type.as_ref().clone());
    self
  }
}

impl AsRef<StoryInteractionTypeView> for StoryInteractionTypeView {
  fn as_ref(&self) -> &StoryInteractionTypeView {
    self
  }
}

impl AsRef<StoryInteractionTypeView> for RTDStoryInteractionTypeViewBuilder {
  fn as_ref(&self) -> &StoryInteractionTypeView {
    &self.inner
  }
}
