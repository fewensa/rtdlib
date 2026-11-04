use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an emoji category
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Name of the category
  name: String,
  /// Custom emoji sticker, which represents icon of the category
  icon: Sticker,
  /// Source of stickers for the emoji category
  source: EmojiCategorySource,
  /// True, if the category must be shown first when choosing a sticker for the start page
  is_greeting: bool,
}

impl RObject for EmojiCategory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiCategory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiCategory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiCategoryBuilder {
    let inner = EmojiCategory {
      td_name: "emojiCategory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiCategoryBuilder { inner }
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn icon(&self) -> &Sticker {
    &self.icon
  }

  pub fn source(&self) -> &EmojiCategorySource {
    &self.source
  }

  pub fn is_greeting(&self) -> bool {
    self.is_greeting
  }
}

#[doc(hidden)]
pub struct RTDEmojiCategoryBuilder {
  inner: EmojiCategory,
}

impl RTDEmojiCategoryBuilder {
  pub fn build(&self) -> EmojiCategory {
    self.inner.clone()
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn icon<T: AsRef<Sticker>>(&mut self, icon: T) -> &mut Self {
    self.inner.icon = icon.as_ref().clone();
    self
  }

  pub fn source<T: AsRef<EmojiCategorySource>>(&mut self, source: T) -> &mut Self {
    self.inner.source = source.as_ref().clone();
    self
  }

  pub fn is_greeting(&mut self, is_greeting: bool) -> &mut Self {
    self.inner.is_greeting = is_greeting;
    self
  }
}

impl AsRef<EmojiCategory> for EmojiCategory {
  fn as_ref(&self) -> &EmojiCategory {
    self
  }
}

impl AsRef<EmojiCategory> for RTDEmojiCategoryBuilder {
  fn as_ref(&self) -> &EmojiCategory {
    &self.inner
  }
}
