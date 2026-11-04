use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of emoji categories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EmojiCategories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of categories
  categories: Vec<EmojiCategory>,
}

impl RObject for EmojiCategories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "emojiCategories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl EmojiCategories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDEmojiCategoriesBuilder {
    let inner = EmojiCategories {
      td_name: "emojiCategories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDEmojiCategoriesBuilder { inner }
  }

  pub fn categories(&self) -> &Vec<EmojiCategory> {
    &self.categories
  }
}

#[doc(hidden)]
pub struct RTDEmojiCategoriesBuilder {
  inner: EmojiCategories,
}

impl RTDEmojiCategoriesBuilder {
  pub fn build(&self) -> EmojiCategories {
    self.inner.clone()
  }

  pub fn categories(&mut self, categories: Vec<EmojiCategory>) -> &mut Self {
    self.inner.categories = categories;
    self
  }
}

impl AsRef<EmojiCategories> for EmojiCategories {
  fn as_ref(&self) -> &EmojiCategories {
    self
  }
}

impl AsRef<EmojiCategories> for RTDEmojiCategoriesBuilder {
  fn as_ref(&self) -> &EmojiCategories {
    &self.inner
  }
}
