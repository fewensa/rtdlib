use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes one answer option of a poll to be created
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPollOption {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Option text; 1-100 characters. Only custom emoji entities are allowed to be added and only by Premium users
  text: FormattedText,
  /// Option media; pass null if none. Must be one of the following types: inputPollMediaAnimation, inputPollMediaLink, inputPollMediaLocation, inputPollMediaPhoto, inputPollMediaSticker, inputPollMediaVenue, or inputPollMediaVideo without caption
  media: InputPollMedia,
}

impl RObject for InputPollOption {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPollOption"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputPollOption {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPollOptionBuilder {
    let inner = InputPollOption {
      td_name: "inputPollOption".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPollOptionBuilder { inner }
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn media(&self) -> &InputPollMedia {
    &self.media
  }
}

#[doc(hidden)]
pub struct RTDInputPollOptionBuilder {
  inner: InputPollOption,
}

impl RTDInputPollOptionBuilder {
  pub fn build(&self) -> InputPollOption {
    self.inner.clone()
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn media<T: AsRef<InputPollMedia>>(&mut self, media: T) -> &mut Self {
    self.inner.media = media.as_ref().clone();
    self
  }
}

impl AsRef<InputPollOption> for InputPollOption {
  fn as_ref(&self) -> &InputPollOption {
    self
  }
}

impl AsRef<InputPollOption> for RTDInputPollOptionBuilder {
  fn as_ref(&self) -> &InputPollOption {
    &self.inner
  }
}
