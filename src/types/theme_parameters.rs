use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains parameters of the application theme
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A color of the background in the RGB format
  background_color: i64,
  /// A secondary color for the background in the RGB format
  secondary_background_color: i64,
  /// A color of the header background in the RGB format
  header_background_color: i64,
  /// A color of the bottom bar background in the RGB format
  bottom_bar_background_color: i64,
  /// A color of the section background in the RGB format
  section_background_color: i64,
  /// A color of the section separator in the RGB format
  section_separator_color: i64,
  /// A color of text in the RGB format
  text_color: i64,
  /// An accent color of the text in the RGB format
  accent_text_color: i64,
  /// A color of text on the section headers in the RGB format
  section_header_text_color: i64,
  /// A color of the subtitle text in the RGB format
  subtitle_text_color: i64,
  /// A color of the text for destructive actions in the RGB format
  destructive_text_color: i64,
  /// A color of hints in the RGB format
  hint_color: i64,
  /// A color of links in the RGB format
  link_color: i64,
  /// A color of the buttons in the RGB format
  button_color: i64,
  /// A color of text on the buttons in the RGB format
  button_text_color: i64,
}

impl RObject for ThemeParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "themeParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ThemeParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDThemeParametersBuilder {
    let inner = ThemeParameters {
      td_name: "themeParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDThemeParametersBuilder { inner }
  }

  pub fn background_color(&self) -> i64 {
    self.background_color
  }

  pub fn secondary_background_color(&self) -> i64 {
    self.secondary_background_color
  }

  pub fn header_background_color(&self) -> i64 {
    self.header_background_color
  }

  pub fn bottom_bar_background_color(&self) -> i64 {
    self.bottom_bar_background_color
  }

  pub fn section_background_color(&self) -> i64 {
    self.section_background_color
  }

  pub fn section_separator_color(&self) -> i64 {
    self.section_separator_color
  }

  pub fn text_color(&self) -> i64 {
    self.text_color
  }

  pub fn accent_text_color(&self) -> i64 {
    self.accent_text_color
  }

  pub fn section_header_text_color(&self) -> i64 {
    self.section_header_text_color
  }

  pub fn subtitle_text_color(&self) -> i64 {
    self.subtitle_text_color
  }

  pub fn destructive_text_color(&self) -> i64 {
    self.destructive_text_color
  }

  pub fn hint_color(&self) -> i64 {
    self.hint_color
  }

  pub fn link_color(&self) -> i64 {
    self.link_color
  }

  pub fn button_color(&self) -> i64 {
    self.button_color
  }

  pub fn button_text_color(&self) -> i64 {
    self.button_text_color
  }
}

#[doc(hidden)]
pub struct RTDThemeParametersBuilder {
  inner: ThemeParameters,
}

impl RTDThemeParametersBuilder {
  pub fn build(&self) -> ThemeParameters {
    self.inner.clone()
  }

  pub fn background_color(&mut self, background_color: i64) -> &mut Self {
    self.inner.background_color = background_color;
    self
  }

  pub fn secondary_background_color(&mut self, secondary_background_color: i64) -> &mut Self {
    self.inner.secondary_background_color = secondary_background_color;
    self
  }

  pub fn header_background_color(&mut self, header_background_color: i64) -> &mut Self {
    self.inner.header_background_color = header_background_color;
    self
  }

  pub fn bottom_bar_background_color(&mut self, bottom_bar_background_color: i64) -> &mut Self {
    self.inner.bottom_bar_background_color = bottom_bar_background_color;
    self
  }

  pub fn section_background_color(&mut self, section_background_color: i64) -> &mut Self {
    self.inner.section_background_color = section_background_color;
    self
  }

  pub fn section_separator_color(&mut self, section_separator_color: i64) -> &mut Self {
    self.inner.section_separator_color = section_separator_color;
    self
  }

  pub fn text_color(&mut self, text_color: i64) -> &mut Self {
    self.inner.text_color = text_color;
    self
  }

  pub fn accent_text_color(&mut self, accent_text_color: i64) -> &mut Self {
    self.inner.accent_text_color = accent_text_color;
    self
  }

  pub fn section_header_text_color(&mut self, section_header_text_color: i64) -> &mut Self {
    self.inner.section_header_text_color = section_header_text_color;
    self
  }

  pub fn subtitle_text_color(&mut self, subtitle_text_color: i64) -> &mut Self {
    self.inner.subtitle_text_color = subtitle_text_color;
    self
  }

  pub fn destructive_text_color(&mut self, destructive_text_color: i64) -> &mut Self {
    self.inner.destructive_text_color = destructive_text_color;
    self
  }

  pub fn hint_color(&mut self, hint_color: i64) -> &mut Self {
    self.inner.hint_color = hint_color;
    self
  }

  pub fn link_color(&mut self, link_color: i64) -> &mut Self {
    self.inner.link_color = link_color;
    self
  }

  pub fn button_color(&mut self, button_color: i64) -> &mut Self {
    self.inner.button_color = button_color;
    self
  }

  pub fn button_text_color(&mut self, button_text_color: i64) -> &mut Self {
    self.inner.button_text_color = button_text_color;
    self
  }
}

impl AsRef<ThemeParameters> for ThemeParameters {
  fn as_ref(&self) -> &ThemeParameters {
    self
  }
}

impl AsRef<ThemeParameters> for RTDThemeParametersBuilder {
  fn as_ref(&self) -> &ThemeParameters {
    &self.inner
  }
}
