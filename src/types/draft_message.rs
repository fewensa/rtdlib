use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a message draft
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DraftMessage {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the message to be replied; inputMessageReplyToStory is unsupported; may be null if none
  reply_to: Option<InputMessageReplyTo>,
  /// Point in time (Unix timestamp) when the draft was created
  date: i64,
  /// Content of the message draft
  content: DraftMessageContent,
  /// Identifier of the effect to apply to the message when it is sent; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  effect_id: isize,
  /// Information about the suggested post; may be null if none
  suggested_post_info: Option<InputSuggestedPostInfo>,
}

impl RObject for DraftMessage {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "draftMessage"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl DraftMessage {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDDraftMessageBuilder {
    let inner = DraftMessage {
      td_name: "draftMessage".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDDraftMessageBuilder { inner }
  }

  pub fn reply_to(&self) -> &Option<InputMessageReplyTo> {
    &self.reply_to
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn content(&self) -> &DraftMessageContent {
    &self.content
  }

  pub fn effect_id(&self) -> isize {
    self.effect_id
  }

  pub fn suggested_post_info(&self) -> &Option<InputSuggestedPostInfo> {
    &self.suggested_post_info
  }
}

#[doc(hidden)]
pub struct RTDDraftMessageBuilder {
  inner: DraftMessage,
}

impl RTDDraftMessageBuilder {
  pub fn build(&self) -> DraftMessage {
    self.inner.clone()
  }

  pub fn reply_to<T: AsRef<InputMessageReplyTo>>(&mut self, reply_to: T) -> &mut Self {
    self.inner.reply_to = Some(reply_to.as_ref().clone());
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn content<T: AsRef<DraftMessageContent>>(&mut self, content: T) -> &mut Self {
    self.inner.content = content.as_ref().clone();
    self
  }

  pub fn effect_id(&mut self, effect_id: isize) -> &mut Self {
    self.inner.effect_id = effect_id;
    self
  }

  pub fn suggested_post_info<T: AsRef<InputSuggestedPostInfo>>(
    &mut self,
    suggested_post_info: T,
  ) -> &mut Self {
    self.inner.suggested_post_info = Some(suggested_post_info.as_ref().clone());
    self
  }
}

impl AsRef<DraftMessage> for DraftMessage {
  fn as_ref(&self) -> &DraftMessage {
    self
  }
}

impl AsRef<DraftMessage> for RTDDraftMessageBuilder {
  fn as_ref(&self) -> &DraftMessage {
    &self.inner
  }
}
