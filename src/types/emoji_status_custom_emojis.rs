use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of custom emoji identifiers for emoji statuses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiStatusCustomEmojis {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of custom emoji identifiers
  custom_emoji_ids: Vec<isize>,
}

impl RObject for EmojiStatusCustomEmojis {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiStatusCustomEmojis"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiStatusCustomEmojis {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiStatusCustomEmojisBuilder {
    let inner = EmojiStatusCustomEmojis {
      td_name: "emojiStatusCustomEmojis".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiStatusCustomEmojisBuilder { inner }
  }

  pub fn custom_emoji_ids(&self) -> &Vec<isize> {
    &self.custom_emoji_ids
  }
}

#[doc(hidden)]
pub struct RTDEmojiStatusCustomEmojisBuilder {
  inner: EmojiStatusCustomEmojis,
}

impl RTDEmojiStatusCustomEmojisBuilder {
  pub fn build(&self) -> EmojiStatusCustomEmojis {
    self.inner.clone()
  }

  pub fn custom_emoji_ids(&mut self, custom_emoji_ids: Vec<isize>) -> &mut Self {
    self.inner.custom_emoji_ids = custom_emoji_ids;
    self
  }
}

impl AsRef<EmojiStatusCustomEmojis> for EmojiStatusCustomEmojis {
  fn as_ref(&self) -> &EmojiStatusCustomEmojis {
    self
  }
}

impl AsRef<EmojiStatusCustomEmojis> for RTDEmojiStatusCustomEmojisBuilder {
  fn as_ref(&self) -> &EmojiStatusCustomEmojis {
    &self.inner
  }
}
