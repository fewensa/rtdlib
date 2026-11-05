use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a media to be used in a sent rich message
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputRichMessageMedia {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the media; 1-64 base64url characters
  id: String,
  /// The media to send. Must be one of the following types: inputMessageAnimation, inputMessageAudio, inputMessagePhoto, inputMessageVideo, or inputMessageVoiceNote
  media: InputMessageContent,
}

impl RObject for InputRichMessageMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputRichMessageMedia"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputRichMessageMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputRichMessageMediaBuilder {
    let inner = InputRichMessageMedia {
      td_name: "inputRichMessageMedia".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputRichMessageMediaBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn media(&self) -> &InputMessageContent {
    &self.media
  }
}

#[doc(hidden)]
pub struct RTDInputRichMessageMediaBuilder {
  inner: InputRichMessageMedia,
}

impl RTDInputRichMessageMediaBuilder {
  pub fn build(&self) -> InputRichMessageMedia {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn media<T: AsRef<InputMessageContent>>(&mut self, media: T) -> &mut Self {
    self.inner.media = media.as_ref().clone();
    self
  }
}

impl AsRef<InputRichMessageMedia> for InputRichMessageMedia {
  fn as_ref(&self) -> &InputRichMessageMedia {
    self
  }
}

impl AsRef<InputRichMessageMedia> for RTDInputRichMessageMediaBuilder {
  fn as_ref(&self) -> &InputRichMessageMedia {
    &self.inner
  }
}
