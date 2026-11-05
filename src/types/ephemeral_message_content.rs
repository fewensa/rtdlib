use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an ephemeral content of a regular message, which must be shown instead of the regular content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EphemeralMessageContent {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if content of the message can be saved locally
  can_be_saved: bool,
  /// True, if media timestamp entities refers to a media in this message as opposed to a media in the replied message
  has_timestamped_media: bool,
  /// Content of the message
  content: MessageContent,
  /// Reply markup for the message; may be null if none
  reply_markup: Option<ReplyMarkup>,
}

impl RObject for EphemeralMessageContent {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "ephemeralMessageContent"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EphemeralMessageContent {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEphemeralMessageContentBuilder {
    let inner = EphemeralMessageContent {
      td_name: "ephemeralMessageContent".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEphemeralMessageContentBuilder { inner }
  }

  pub fn can_be_saved(&self) -> bool {
    self.can_be_saved
  }

  pub fn has_timestamped_media(&self) -> bool {
    self.has_timestamped_media
  }

  pub fn content(&self) -> &MessageContent {
    &self.content
  }

  pub fn reply_markup(&self) -> &Option<ReplyMarkup> {
    &self.reply_markup
  }
}

#[doc(hidden)]
pub struct RTDEphemeralMessageContentBuilder {
  inner: EphemeralMessageContent,
}

impl RTDEphemeralMessageContentBuilder {
  pub fn build(&self) -> EphemeralMessageContent {
    self.inner.clone()
  }

  pub fn can_be_saved(&mut self, can_be_saved: bool) -> &mut Self {
    self.inner.can_be_saved = can_be_saved;
    self
  }

  pub fn has_timestamped_media(&mut self, has_timestamped_media: bool) -> &mut Self {
    self.inner.has_timestamped_media = has_timestamped_media;
    self
  }

  pub fn content<T: AsRef<MessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

  pub fn reply_markup<T: AsRef<ReplyMarkup>>(&mut self, reply_markup: T) -> &mut Self {
    self.inner.reply_markup = Some(reply_markup.as_ref().clone());
    self
  }
}

impl AsRef<EphemeralMessageContent> for EphemeralMessageContent {
  fn as_ref(&self) -> &EphemeralMessageContent {
    self
  }
}

impl AsRef<EphemeralMessageContent> for RTDEphemeralMessageContentBuilder {
  fn as_ref(&self) -> &EphemeralMessageContent {
    &self.inner
  }
}
