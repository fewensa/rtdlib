use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about supported accent colors for user profile photo background in RGB format
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileAccentColors {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of 1-2 colors in RGB format, describing the colors, as expected to be shown in the color palette settings
  palette_colors: Vec<i64>,
  /// The list of 1-2 colors in RGB format, describing the colors, as expected to be used for the profile photo background
  background_colors: Vec<i64>,
  /// The list of 2 colors in RGB format, describing the colors of the gradient to be used for the unread active story indicator around profile photo
  story_colors: Vec<i64>,
}

impl RObject for ProfileAccentColors {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileAccentColors"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ProfileAccentColors {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileAccentColorsBuilder {
    let inner = ProfileAccentColors {
      td_name: "profileAccentColors".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDProfileAccentColorsBuilder { inner }
  }

  pub fn palette_colors(&self) -> &Vec<i64> {
    &self.palette_colors
  }

  pub fn background_colors(&self) -> &Vec<i64> {
    &self.background_colors
  }

  pub fn story_colors(&self) -> &Vec<i64> {
    &self.story_colors
  }
}

#[doc(hidden)]
pub struct RTDProfileAccentColorsBuilder {
  inner: ProfileAccentColors,
}

impl RTDProfileAccentColorsBuilder {
  pub fn build(&self) -> ProfileAccentColors {
    self.inner.clone()
  }

  pub fn palette_colors(&mut self, palette_colors: Vec<i64>) -> &mut Self {
    self.inner.palette_colors = palette_colors;
    self
  }

  pub fn background_colors(&mut self, background_colors: Vec<i64>) -> &mut Self {
    self.inner.background_colors = background_colors;
    self
  }

  pub fn story_colors(&mut self, story_colors: Vec<i64>) -> &mut Self {
    self.inner.story_colors = story_colors;
    self
  }
}

impl AsRef<ProfileAccentColors> for ProfileAccentColors {
  fn as_ref(&self) -> &ProfileAccentColors {
    self
  }
}

impl AsRef<ProfileAccentColors> for RTDProfileAccentColorsBuilder {
  fn as_ref(&self) -> &ProfileAccentColors {
    &self.inner
  }
}
