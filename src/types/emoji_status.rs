use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an emoji to be shown instead of the Telegram Premium badge
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiStatus {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the emoji status
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: EmojiStatusType,
  /// Point in time (Unix timestamp) when the status will expire; 0 if never
  expiration_date: i64,
}

impl RObject for EmojiStatus {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiStatus"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiStatus {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiStatusBuilder {
    let inner = EmojiStatus {
      td_name: "emojiStatus".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiStatusBuilder { inner }
  }

  pub fn type_(&self) -> &EmojiStatusType {
    &self.type_
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }
}

#[doc(hidden)]
pub struct RTDEmojiStatusBuilder {
  inner: EmojiStatus,
}

impl RTDEmojiStatusBuilder {
  pub fn build(&self) -> EmojiStatus {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<EmojiStatusType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }
}

impl AsRef<EmojiStatus> for EmojiStatus {
  fn as_ref(&self) -> &EmojiStatus {
    self
  }
}

impl AsRef<EmojiStatus> for RTDEmojiStatusBuilder {
  fn as_ref(&self) -> &EmojiStatus {
    &self.inner
  }
}
