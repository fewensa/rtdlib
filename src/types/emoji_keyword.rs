use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents an emoji with its keyword
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiKeyword {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The emoji
  emoji: String,
  /// The keyword
  keyword: String,
}

impl RObject for EmojiKeyword {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiKeyword"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiKeyword {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiKeywordBuilder {
    let inner = EmojiKeyword {
      td_name: "emojiKeyword".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiKeywordBuilder { inner }
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }

  pub fn keyword(&self) -> &String {
    &self.keyword
  }
}

#[doc(hidden)]
pub struct RTDEmojiKeywordBuilder {
  inner: EmojiKeyword,
}

impl RTDEmojiKeywordBuilder {
  pub fn build(&self) -> EmojiKeyword {
    self.inner.clone()
  }

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

  pub fn keyword<T: AsRef<str>>(&mut self, keyword: T) -> &mut Self {
    self.inner.keyword = keyword.as_ref().to_string();
    self
  }
}

impl AsRef<EmojiKeyword> for EmojiKeyword {
  fn as_ref(&self) -> &EmojiKeyword {
    self
  }
}

impl AsRef<EmojiKeyword> for RTDEmojiKeywordBuilder {
  fn as_ref(&self) -> &EmojiKeyword {
    &self.inner
  }
}
