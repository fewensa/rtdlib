use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of emojis with their keywords
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiKeywords {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of emojis with their keywords
  emoji_keywords: Vec<EmojiKeyword>,
}

impl RObject for EmojiKeywords {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiKeywords"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiKeywords {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiKeywordsBuilder {
    let inner = EmojiKeywords {
      td_name: "emojiKeywords".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiKeywordsBuilder { inner }
  }

  pub fn emoji_keywords(&self) -> &Vec<EmojiKeyword> {
    &self.emoji_keywords
  }
}

#[doc(hidden)]
pub struct RTDEmojiKeywordsBuilder {
  inner: EmojiKeywords,
}

impl RTDEmojiKeywordsBuilder {
  pub fn build(&self) -> EmojiKeywords {
    self.inner.clone()
  }

  pub fn emoji_keywords(&mut self, emoji_keywords: Vec<EmojiKeyword>) -> &mut Self {
    self.inner.emoji_keywords = emoji_keywords;
    self
  }
}

impl AsRef<EmojiKeywords> for EmojiKeywords {
  fn as_ref(&self) -> &EmojiKeywords {
    self
  }
}

impl AsRef<EmojiKeywords> for RTDEmojiKeywordsBuilder {
  fn as_ref(&self) -> &EmojiKeywords {
    &self.inner
  }
}
