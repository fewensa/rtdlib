use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the type of poll to send
pub trait TDInputPollType: Debug + RObject {}

/// Describes the type of poll to send
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputPollType {
  #[doc(hidden)]
  _Default(()),
  /// A poll in quiz mode, which has predefined correct answers
  Quiz(InputPollTypeQuiz),
  /// A regular poll
  Regular(InputPollTypeRegular),
}

impl Default for InputPollType {
  fn default() -> Self {
    InputPollType::_Default(())
  }
}

impl<'de> Deserialize<'de> for InputPollType {
  fn deserialize<D>(deserializer: D) -> Result<InputPollType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InputPollType,
      (inputPollTypeQuiz, Quiz);
      (inputPollTypeRegular, Regular);

    )(deserializer)
  }
}

impl RObject for InputPollType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InputPollType::Quiz(t) => t.td_name(),
      InputPollType::Regular(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InputPollType::Quiz(t) => t.extra(),
      InputPollType::Regular(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputPollType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InputPollType::_Default(_))
  }

  pub fn is_quiz(&self) -> bool {
    matches!(self, InputPollType::Quiz(_))
  }
  pub fn is_regular(&self) -> bool {
    matches!(self, InputPollType::Regular(_))
  }

  pub fn on_quiz<F: FnOnce(&InputPollTypeQuiz)>(&self, fnc: F) -> &Self {
    if let InputPollType::Quiz(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_regular<F: FnOnce(&InputPollTypeRegular)>(&self, fnc: F) -> &Self {
    if let InputPollType::Regular(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_quiz(&self) -> Option<&InputPollTypeQuiz> {
    if let InputPollType::Quiz(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_regular(&self) -> Option<&InputPollTypeRegular> {
    if let InputPollType::Regular(t) = self {
      return Some(t);
    }
    None
  }

  pub fn quiz<T: AsRef<InputPollTypeQuiz>>(t: T) -> Self {
    InputPollType::Quiz(t.as_ref().clone())
  }

  pub fn regular<T: AsRef<InputPollTypeRegular>>(t: T) -> Self {
    InputPollType::Regular(t.as_ref().clone())
  }
}

impl AsRef<InputPollType> for InputPollType {
  fn as_ref(&self) -> &InputPollType {
    self
  }
}

/// A poll in quiz mode, which has predefined correct answers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollTypeQuiz {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Increasing list of 0-based identifiers of the correct answer options; must be non-empty
  correct_option_ids: Vec<i64>,
  /// Text that is shown when the user chooses an incorrect answer or taps on the lamp icon; 0-200 characters with at most 2 line feeds
  explanation: FormattedText,
  /// Media that is shown when the user chooses an incorrect answer or taps on the lamp icon; pass null if none. Must be one of the following types: inputPollMediaAnimation, inputPollMediaAudio, inputPollMediaDocument, inputPollMediaLocation, inputPollMediaPhoto, inputPollMediaVenue, or inputPollMediaVideo without caption
  explanation_media: InputPollMedia,
}

impl RObject for InputPollTypeQuiz {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollTypeQuiz"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollType for InputPollTypeQuiz {}

impl InputPollTypeQuiz {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollTypeQuizBuilder {
    let inner = InputPollTypeQuiz {
      td_name: "inputPollTypeQuiz".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollTypeQuizBuilder { inner }
  }

  pub fn correct_option_ids(&self) -> &Vec<i64> {
    &self.correct_option_ids
  }

  pub fn explanation(&self) -> &FormattedText {
    &self.explanation
  }

  pub fn explanation_media(&self) -> &InputPollMedia {
    &self.explanation_media
  }
}

#[doc(hidden)]
pub struct RTDInputPollTypeQuizBuilder {
  inner: InputPollTypeQuiz,
}

impl RTDInputPollTypeQuizBuilder {
  pub fn build(&self) -> InputPollTypeQuiz {
    self.inner.clone()
  }

  pub fn correct_option_ids(&mut self, correct_option_ids: Vec<i64>) -> &mut Self {
    self.inner.correct_option_ids = correct_option_ids;
    self
  }

  pub fn explanation<T: AsRef<FormattedText>>(&mut self, explanation: T) -> &mut Self {
    self.inner.explanation = explanation.as_ref().clone();
    self
  }

  pub fn explanation_media<T: AsRef<InputPollMedia>>(&mut self, explanation_media: T) -> &mut Self {
    self.inner.explanation_media = explanation_media.as_ref().clone();
    self
  }
}

impl AsRef<InputPollTypeQuiz> for InputPollTypeQuiz {
  fn as_ref(&self) -> &InputPollTypeQuiz {
    self
  }
}

impl AsRef<InputPollTypeQuiz> for RTDInputPollTypeQuizBuilder {
  fn as_ref(&self) -> &InputPollTypeQuiz {
    &self.inner
  }
}

/// A regular poll
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollTypeRegular {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if answer options can be added to the poll after creation; not supported in channel chats and for anonymous polls
  allow_adding_options: bool,
}

impl RObject for InputPollTypeRegular {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollTypeRegular"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInputPollType for InputPollTypeRegular {}

impl InputPollTypeRegular {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollTypeRegularBuilder {
    let inner = InputPollTypeRegular {
      td_name: "inputPollTypeRegular".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollTypeRegularBuilder { inner }
  }

  pub fn allow_adding_options(&self) -> bool {
    self.allow_adding_options
  }
}

#[doc(hidden)]
pub struct RTDInputPollTypeRegularBuilder {
  inner: InputPollTypeRegular,
}

impl RTDInputPollTypeRegularBuilder {
  pub fn build(&self) -> InputPollTypeRegular {
    self.inner.clone()
  }

  pub fn allow_adding_options(&mut self, allow_adding_options: bool) -> &mut Self {
    self.inner.allow_adding_options = allow_adding_options;
    self
  }
}

impl AsRef<InputPollTypeRegular> for InputPollTypeRegular {
  fn as_ref(&self) -> &InputPollTypeRegular {
    self
  }
}

impl AsRef<InputPollTypeRegular> for RTDInputPollTypeRegularBuilder {
  fn as_ref(&self) -> &InputPollTypeRegular {
    &self.inner
  }
}
