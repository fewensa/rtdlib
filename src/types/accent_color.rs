use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about supported accent color for user/chat name, background of empty chat photo, replies to messages and link previews
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccentColor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Accent color identifier
  id: i64,
  /// Identifier of a built-in color to use in places, where only one color is needed; 0-6
  built_in_accent_color_id: i64,
  /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in light themes
  light_theme_colors: Vec<i64>,
  /// The list of 1-3 colors in RGB format, describing the accent color, as expected to be shown in dark themes
  dark_theme_colors: Vec<i64>,
  /// The minimum chat boost level required to use the color in a channel chat
  min_channel_chat_boost_level: i64,
}

impl RObject for AccentColor {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "accentColor"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AccentColor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAccentColorBuilder {
    let inner = AccentColor {
      td_name: "accentColor".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAccentColorBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn built_in_accent_color_id(&self) -> i64 {
    self.built_in_accent_color_id
  }

  pub fn light_theme_colors(&self) -> &Vec<i64> {
    &self.light_theme_colors
  }

  pub fn dark_theme_colors(&self) -> &Vec<i64> {
    &self.dark_theme_colors
  }

  pub fn min_channel_chat_boost_level(&self) -> i64 {
    self.min_channel_chat_boost_level
  }
}

#[doc(hidden)]
pub struct RTDAccentColorBuilder {
  inner: AccentColor,
}

impl RTDAccentColorBuilder {
  pub fn build(&self) -> AccentColor {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn built_in_accent_color_id(&mut self, built_in_accent_color_id: i64) -> &mut Self {
    self.inner.built_in_accent_color_id = built_in_accent_color_id;
    self
  }

  pub fn light_theme_colors(&mut self, light_theme_colors: Vec<i64>) -> &mut Self {
    self.inner.light_theme_colors = light_theme_colors;
    self
  }

  pub fn dark_theme_colors(&mut self, dark_theme_colors: Vec<i64>) -> &mut Self {
    self.inner.dark_theme_colors = dark_theme_colors;
    self
  }

  pub fn min_channel_chat_boost_level(&mut self, min_channel_chat_boost_level: i64) -> &mut Self {
    self.inner.min_channel_chat_boost_level = min_channel_chat_boost_level;
    self
  }
}

impl AsRef<AccentColor> for AccentColor {
  fn as_ref(&self) -> &AccentColor {
    self
  }
}

impl AsRef<AccentColor> for RTDAccentColorBuilder {
  fn as_ref(&self) -> &AccentColor {
    &self.inner
  }
}
