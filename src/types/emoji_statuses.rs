use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of emoji statuses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiStatuses {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of emoji statuses identifiers
  emoji_statuses: Vec<EmojiStatus>,
}

impl RObject for EmojiStatuses {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiStatuses"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiStatuses {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiStatusesBuilder {
    let inner = EmojiStatuses {
      td_name: "emojiStatuses".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiStatusesBuilder { inner }
  }

  pub fn emoji_statuses(&self) -> &Vec<EmojiStatus> {
    &self.emoji_statuses
  }
}

#[doc(hidden)]
pub struct RTDEmojiStatusesBuilder {
  inner: EmojiStatuses,
}

impl RTDEmojiStatusesBuilder {
  pub fn build(&self) -> EmojiStatuses {
    self.inner.clone()
  }

  pub fn emoji_statuses(&mut self, emoji_statuses: Vec<EmojiStatus>) -> &mut Self {
    self.inner.emoji_statuses = emoji_statuses;
    self
  }
}

impl AsRef<EmojiStatuses> for EmojiStatuses {
  fn as_ref(&self) -> &EmojiStatuses {
    self
  }
}

impl AsRef<EmojiStatuses> for RTDEmojiStatusesBuilder {
  fn as_ref(&self) -> &EmojiStatuses {
    &self.inner
  }
}
