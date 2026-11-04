use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about supported accent color for user profile photo background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileAccentColor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Profile accent color identifier
  id: i64,
  /// Accent colors expected to be used in light themes
  light_theme_colors: ProfileAccentColors,
  /// Accent colors expected to be used in dark themes
  dark_theme_colors: ProfileAccentColors,
  /// The minimum chat boost level required to use the color in a supergroup chat
  min_supergroup_chat_boost_level: i64,
  /// The minimum chat boost level required to use the color in a channel chat
  min_channel_chat_boost_level: i64,
}

impl RObject for ProfileAccentColor {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileAccentColor"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ProfileAccentColor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileAccentColorBuilder {
    let inner = ProfileAccentColor {
      td_name: "profileAccentColor".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDProfileAccentColorBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn light_theme_colors(&self) -> &ProfileAccentColors {
    &self.light_theme_colors
  }

  pub fn dark_theme_colors(&self) -> &ProfileAccentColors {
    &self.dark_theme_colors
  }

  pub fn min_supergroup_chat_boost_level(&self) -> i64 {
    self.min_supergroup_chat_boost_level
  }

  pub fn min_channel_chat_boost_level(&self) -> i64 {
    self.min_channel_chat_boost_level
  }
}

#[doc(hidden)]
pub struct RTDProfileAccentColorBuilder {
  inner: ProfileAccentColor,
}

impl RTDProfileAccentColorBuilder {
  pub fn build(&self) -> ProfileAccentColor {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn light_theme_colors<T: AsRef<ProfileAccentColors>>(
    &mut self,
    light_theme_colors: T,
  ) -> &mut Self {
    self.inner.light_theme_colors = light_theme_colors.as_ref().clone();
    self
  }

  pub fn dark_theme_colors<T: AsRef<ProfileAccentColors>>(
    &mut self,
    dark_theme_colors: T,
  ) -> &mut Self {
    self.inner.dark_theme_colors = dark_theme_colors.as_ref().clone();
    self
  }

  pub fn min_supergroup_chat_boost_level(
    &mut self,
    min_supergroup_chat_boost_level: i64,
  ) -> &mut Self {
    self.inner.min_supergroup_chat_boost_level = min_supergroup_chat_boost_level;
    self
  }

  pub fn min_channel_chat_boost_level(&mut self, min_channel_chat_boost_level: i64) -> &mut Self {
    self.inner.min_channel_chat_boost_level = min_channel_chat_boost_level;
    self
  }
}

impl AsRef<ProfileAccentColor> for ProfileAccentColor {
  fn as_ref(&self) -> &ProfileAccentColor {
    self
  }
}

impl AsRef<ProfileAccentColor> for RTDProfileAccentColorBuilder {
  fn as_ref(&self) -> &ProfileAccentColor {
    &self.inner
  }
}
