use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes the original details about the gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftOriginalDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user or the chat that sent the gift; may be null if the gift was private
  sender_id: Option<MessageSender>,
  /// Identifier of the user or the chat that received the gift
  receiver_id: MessageSender,
  /// Message added to the gift
  text: FormattedText,
  /// Point in time (Unix timestamp) when the gift was sent
  date: i64,
}

impl RObject for UpgradedGiftOriginalDetails {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftOriginalDetails"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftOriginalDetails {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftOriginalDetailsBuilder {
    let inner = UpgradedGiftOriginalDetails {
      td_name: "upgradedGiftOriginalDetails".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftOriginalDetailsBuilder { inner }
  }

  pub fn sender_id(&self) -> &Option<MessageSender> {
    &self.sender_id
  }

  pub fn receiver_id(&self) -> &MessageSender {
    &self.receiver_id
  }

  pub fn text(&self) -> &FormattedText {
    &self.text
  }

  pub fn date(&self) -> i64 {
    self.date
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftOriginalDetailsBuilder {
  inner: UpgradedGiftOriginalDetails,
}

impl RTDUpgradedGiftOriginalDetailsBuilder {
  pub fn build(&self) -> UpgradedGiftOriginalDetails {
    self.inner.clone()
  }

  pub fn sender_id<T: AsRef<MessageSender>>(&mut self, sender_id: T) -> &mut Self {
    self.inner.sender_id = Some(sender_id.as_ref().clone());
    self
  }

  pub fn receiver_id<T: AsRef<MessageSender>>(&mut self, receiver_id: T) -> &mut Self {
    self.inner.receiver_id = receiver_id.as_ref().clone();
    self
  }

  pub fn text<T: AsRef<FormattedText>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().clone();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }
}

impl AsRef<UpgradedGiftOriginalDetails> for UpgradedGiftOriginalDetails {
  fn as_ref(&self) -> &UpgradedGiftOriginalDetails {
    self
  }
}

impl AsRef<UpgradedGiftOriginalDetails> for RTDUpgradedGiftOriginalDetailsBuilder {
  fn as_ref(&self) -> &UpgradedGiftOriginalDetails {
    &self.inner
  }
}
