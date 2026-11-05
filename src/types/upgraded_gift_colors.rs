use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about color scheme for user's name, background of empty chat photo, replies to messages and link previews
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftColors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the upgraded gift colors
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Custom emoji identifier of the model of the upgraded gift
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  model_custom_emoji_id: isize,
  /// Custom emoji identifier of the symbol of the upgraded gift
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  symbol_custom_emoji_id: isize,
  /// Accent color to use in light themes in RGB format
  light_theme_accent_color: i64,
  /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in light themes
  light_theme_colors: Vec<i64>,
  /// Accent color to use in dark themes in RGB format
  dark_theme_accent_color: i64,
  /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in dark themes
  dark_theme_colors: Vec<i64>,
}

impl RObject for UpgradedGiftColors {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftColors"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftColors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftColorsBuilder {
    let inner = UpgradedGiftColors {
      td_name: "upgradedGiftColors".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftColorsBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn model_custom_emoji_id(&self) -> isize {
    self.model_custom_emoji_id
  }

  pub fn symbol_custom_emoji_id(&self) -> isize {
    self.symbol_custom_emoji_id
  }

  pub fn light_theme_accent_color(&self) -> i64 {
    self.light_theme_accent_color
  }

  pub fn light_theme_colors(&self) -> &Vec<i64> {
    &self.light_theme_colors
  }

  pub fn dark_theme_accent_color(&self) -> i64 {
    self.dark_theme_accent_color
  }

  pub fn dark_theme_colors(&self) -> &Vec<i64> {
    &self.dark_theme_colors
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftColorsBuilder {
  inner: UpgradedGiftColors,
}

impl RTDUpgradedGiftColorsBuilder {
  pub fn build(&self) -> UpgradedGiftColors {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn model_custom_emoji_id(&mut self, model_custom_emoji_id: isize) -> &mut Self {
    self.inner.model_custom_emoji_id = model_custom_emoji_id;
    self
  }

  pub fn symbol_custom_emoji_id(&mut self, symbol_custom_emoji_id: isize) -> &mut Self {
    self.inner.symbol_custom_emoji_id = symbol_custom_emoji_id;
    self
  }

  pub fn light_theme_accent_color(&mut self, light_theme_accent_color: i64) -> &mut Self {
    self.inner.light_theme_accent_color = light_theme_accent_color;
    self
  }

  pub fn light_theme_colors(&mut self, light_theme_colors: Vec<i64>) -> &mut Self {
    self.inner.light_theme_colors = light_theme_colors;
    self
  }

  pub fn dark_theme_accent_color(&mut self, dark_theme_accent_color: i64) -> &mut Self {
    self.inner.dark_theme_accent_color = dark_theme_accent_color;
    self
  }

  pub fn dark_theme_colors(&mut self, dark_theme_colors: Vec<i64>) -> &mut Self {
    self.inner.dark_theme_colors = dark_theme_colors;
    self
  }
}

impl AsRef<UpgradedGiftColors> for UpgradedGiftColors {
  fn as_ref(&self) -> &UpgradedGiftColors {
    self
  }
}

impl AsRef<UpgradedGiftColors> for RTDUpgradedGiftColorsBuilder {
  fn as_ref(&self) -> &UpgradedGiftColors {
    &self.inner
  }
}
