use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of speech recognition in a voice note
pub trait TDSpeechRecognitionResult: Debug + RObject {}

/// Describes result of speech recognition in a voice note
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SpeechRecognitionResult {
  #[doc(hidden)]
  _Default(()),
  /// The speech recognition failed
  Error(SpeechRecognitionResultError),
  /// The speech recognition is ongoing
  Pending(SpeechRecognitionResultPending),
  /// The speech recognition successfully finished
  Text(SpeechRecognitionResultText),
}

impl Default for SpeechRecognitionResult {
  fn default() -> Self {
    SpeechRecognitionResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for SpeechRecognitionResult {
  fn deserialize<D>(deserializer: D) -> Result<SpeechRecognitionResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SpeechRecognitionResult,
      (speechRecognitionResultError, Error);
      (speechRecognitionResultPending, Pending);
      (speechRecognitionResultText, Text);

    )(deserializer)
  }
}

impl RObject for SpeechRecognitionResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SpeechRecognitionResult::Error(t) => t.td_name(),
      SpeechRecognitionResult::Pending(t) => t.td_name(),
      SpeechRecognitionResult::Text(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SpeechRecognitionResult::Error(t) => t.extra(),
      SpeechRecognitionResult::Pending(t) => t.extra(),
      SpeechRecognitionResult::Text(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SpeechRecognitionResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SpeechRecognitionResult::_Default(_))
  }

  pub fn is_error(&self) -> bool {
    matches!(self, SpeechRecognitionResult::Error(_))
  }
  pub fn is_pending(&self) -> bool {
    matches!(self, SpeechRecognitionResult::Pending(_))
  }
  pub fn is_text(&self) -> bool {
    matches!(self, SpeechRecognitionResult::Text(_))
  }

  pub fn on_error<F: FnOnce(&SpeechRecognitionResultError)>(&self, fnc: F) -> &Self {
    if let SpeechRecognitionResult::Error(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pending<F: FnOnce(&SpeechRecognitionResultPending)>(&self, fnc: F) -> &Self {
    if let SpeechRecognitionResult::Pending(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text<F: FnOnce(&SpeechRecognitionResultText)>(&self, fnc: F) -> &Self {
    if let SpeechRecognitionResult::Text(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_error(&self) -> Option<&SpeechRecognitionResultError> {
    if let SpeechRecognitionResult::Error(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pending(&self) -> Option<&SpeechRecognitionResultPending> {
    if let SpeechRecognitionResult::Pending(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text(&self) -> Option<&SpeechRecognitionResultText> {
    if let SpeechRecognitionResult::Text(t) = self {
      return Some(t);
    }
    None
  }

  pub fn error<T: AsRef<SpeechRecognitionResultError>>(t: T) -> Self {
    SpeechRecognitionResult::Error(t.as_ref().clone())
  }

  pub fn pending<T: AsRef<SpeechRecognitionResultPending>>(t: T) -> Self {
    SpeechRecognitionResult::Pending(t.as_ref().clone())
  }

  pub fn text<T: AsRef<SpeechRecognitionResultText>>(t: T) -> Self {
    SpeechRecognitionResult::Text(t.as_ref().clone())
  }
}

impl AsRef<SpeechRecognitionResult> for SpeechRecognitionResult {
  fn as_ref(&self) -> &SpeechRecognitionResult {
    self
  }
}

/// The speech recognition failed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeechRecognitionResultError {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Recognition error. An error with a message "MSG_VOICE_TOO_LONG" is returned when media duration is too big to be recognized
  error: Error,
}

impl RObject for SpeechRecognitionResultError {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "speechRecognitionResultError"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSpeechRecognitionResult for SpeechRecognitionResultError {}

impl SpeechRecognitionResultError {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSpeechRecognitionResultErrorBuilder {
    let inner = SpeechRecognitionResultError {
      td_name: "speechRecognitionResultError".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSpeechRecognitionResultErrorBuilder { inner }
  }

  pub fn error(&self) -> &Error {
    &self.error
  }
}

#[doc(hidden)]
pub struct RTDSpeechRecognitionResultErrorBuilder {
  inner: SpeechRecognitionResultError,
}

impl RTDSpeechRecognitionResultErrorBuilder {
  pub fn build(&self) -> SpeechRecognitionResultError {
    self.inner.clone()
  }

  pub fn error<T: AsRef<Error>>(&mut self, error: T) -> &mut Self {
    self.inner.error = error.as_ref().clone();
    self
  }
}

impl AsRef<SpeechRecognitionResultError> for SpeechRecognitionResultError {
  fn as_ref(&self) -> &SpeechRecognitionResultError {
    self
  }
}

impl AsRef<SpeechRecognitionResultError> for RTDSpeechRecognitionResultErrorBuilder {
  fn as_ref(&self) -> &SpeechRecognitionResultError {
    &self.inner
  }
}

/// The speech recognition is ongoing
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeechRecognitionResultPending {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Partially recognized text
  partial_text: String,
}

impl RObject for SpeechRecognitionResultPending {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "speechRecognitionResultPending"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSpeechRecognitionResult for SpeechRecognitionResultPending {}

impl SpeechRecognitionResultPending {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSpeechRecognitionResultPendingBuilder {
    let inner = SpeechRecognitionResultPending {
      td_name: "speechRecognitionResultPending".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSpeechRecognitionResultPendingBuilder { inner }
  }

  pub fn partial_text(&self) -> &String {
    &self.partial_text
  }
}

#[doc(hidden)]
pub struct RTDSpeechRecognitionResultPendingBuilder {
  inner: SpeechRecognitionResultPending,
}

impl RTDSpeechRecognitionResultPendingBuilder {
  pub fn build(&self) -> SpeechRecognitionResultPending {
    self.inner.clone()
  }

  pub fn partial_text<T: AsRef<str>>(&mut self, partial_text: T) -> &mut Self {
    self.inner.partial_text = partial_text.as_ref().to_string();
    self
  }
}

impl AsRef<SpeechRecognitionResultPending> for SpeechRecognitionResultPending {
  fn as_ref(&self) -> &SpeechRecognitionResultPending {
    self
  }
}

impl AsRef<SpeechRecognitionResultPending> for RTDSpeechRecognitionResultPendingBuilder {
  fn as_ref(&self) -> &SpeechRecognitionResultPending {
    &self.inner
  }
}

/// The speech recognition successfully finished
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeechRecognitionResultText {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Recognized text
  text: String,
}

impl RObject for SpeechRecognitionResultText {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "speechRecognitionResultText"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSpeechRecognitionResult for SpeechRecognitionResultText {}

impl SpeechRecognitionResultText {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSpeechRecognitionResultTextBuilder {
    let inner = SpeechRecognitionResultText {
      td_name: "speechRecognitionResultText".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSpeechRecognitionResultTextBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }
}

#[doc(hidden)]
pub struct RTDSpeechRecognitionResultTextBuilder {
  inner: SpeechRecognitionResultText,
}

impl RTDSpeechRecognitionResultTextBuilder {
  pub fn build(&self) -> SpeechRecognitionResultText {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }
}

impl AsRef<SpeechRecognitionResultText> for SpeechRecognitionResultText {
  fn as_ref(&self) -> &SpeechRecognitionResultText {
    self
  }
}

impl AsRef<SpeechRecognitionResultText> for RTDSpeechRecognitionResultTextBuilder {
  fn as_ref(&self) -> &SpeechRecognitionResultText {
    &self.inner
  }
}
