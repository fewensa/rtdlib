use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat themes based on upgraded gifts
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftChatThemes {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of chat themes
  themes: Vec<GiftChatTheme>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for GiftChatThemes {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftChatThemes"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftChatThemes {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftChatThemesBuilder {
    let inner = GiftChatThemes {
      td_name: "giftChatThemes".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftChatThemesBuilder { inner }
  }

  pub fn themes(&self) -> &Vec<GiftChatTheme> {
    &self.themes
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDGiftChatThemesBuilder {
  inner: GiftChatThemes,
}

impl RTDGiftChatThemesBuilder {
  pub fn build(&self) -> GiftChatThemes {
    self.inner.clone()
  }

  pub fn themes(&mut self, themes: Vec<GiftChatTheme>) -> &mut Self {
    self.inner.themes = themes;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<GiftChatThemes> for GiftChatThemes {
  fn as_ref(&self) -> &GiftChatThemes {
    self
  }
}

impl AsRef<GiftChatThemes> for RTDGiftChatThemesBuilder {
  fn as_ref(&self) -> &GiftChatThemes {
    &self.inner
  }
}
