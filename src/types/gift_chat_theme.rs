use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a chat theme based on an upgraded gift
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftChatTheme {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The gift
  gift: UpgradedGift,
  /// Theme settings for a light chat theme
  light_settings: ThemeSettings,
  /// Theme settings for a dark chat theme
  dark_settings: ThemeSettings,
}

impl RObject for GiftChatTheme {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftChatTheme"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftChatTheme {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftChatThemeBuilder {
    let inner = GiftChatTheme {
      td_name: "giftChatTheme".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftChatThemeBuilder { inner }
  }

  pub fn gift(&self) -> &UpgradedGift {
    &self.gift
  }

  pub fn light_settings(&self) -> &ThemeSettings {
    &self.light_settings
  }

  pub fn dark_settings(&self) -> &ThemeSettings {
    &self.dark_settings
  }
}

#[doc(hidden)]
pub struct RTDGiftChatThemeBuilder {
  inner: GiftChatTheme,
}

impl RTDGiftChatThemeBuilder {
  pub fn build(&self) -> GiftChatTheme {
    self.inner.clone()
  }

  pub fn gift<T: AsRef<UpgradedGift>>(&mut self, gift: T) -> &mut Self {
    self.inner.gift = gift.as_ref().clone();
    self
  }

  pub fn light_settings<T: AsRef<ThemeSettings>>(&mut self, light_settings: T) -> &mut Self {
    self.inner.light_settings = light_settings.as_ref().clone();
    self
  }

  pub fn dark_settings<T: AsRef<ThemeSettings>>(&mut self, dark_settings: T) -> &mut Self {
    self.inner.dark_settings = dark_settings.as_ref().clone();
    self
  }
}

impl AsRef<GiftChatTheme> for GiftChatTheme {
  fn as_ref(&self) -> &GiftChatTheme {
    self
  }
}

impl AsRef<GiftChatTheme> for RTDGiftChatThemeBuilder {
  fn as_ref(&self) -> &GiftChatTheme {
    &self.inner
  }
}
