
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Describes theme settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThemeSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Theme accent color in ARGB format
  accent_color: i64,
  /// The background to be used in chats; may be null
  background: Option<Background>,
  /// The fill to be used as a background for outgoing messages
  outgoing_message_fill: BackgroundFill,
  /// If true, the freeform gradient fill needs to be animated on every sent message
  animate_outgoing_message_fill: bool,
  /// Accent color of outgoing messages in ARGB format
  outgoing_message_accent_color: i64,
  
}

impl RObject for ThemeSettings {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "themeSettings" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ThemeSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDThemeSettingsBuilder {
    let mut inner = ThemeSettings::default();
    inner.td_name = "themeSettings".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDThemeSettingsBuilder { inner }
  }

  pub fn accent_color(&self) -> i64 { self.accent_color }

  pub fn background(&self) -> &Option<Background> { &self.background }

  pub fn outgoing_message_fill(&self) -> &BackgroundFill { &self.outgoing_message_fill }

  pub fn animate_outgoing_message_fill(&self) -> bool { self.animate_outgoing_message_fill }

  pub fn outgoing_message_accent_color(&self) -> i64 { self.outgoing_message_accent_color }

}

#[doc(hidden)]
pub struct RTDThemeSettingsBuilder {
  inner: ThemeSettings
}

impl RTDThemeSettingsBuilder {
  pub fn build(&self) -> ThemeSettings { self.inner.clone() }

   
  pub fn accent_color(&mut self, accent_color: i64) -> &mut Self {
    self.inner.accent_color = accent_color;
    self
  }

   
  pub fn background<T: AsRef<Background>>(&mut self, background: T) -> &mut Self {
    self.inner.background = Some(background.as_ref().clone());
    self
  }

   
  pub fn outgoing_message_fill<T: AsRef<BackgroundFill>>(&mut self, outgoing_message_fill: T) -> &mut Self {
    self.inner.outgoing_message_fill = outgoing_message_fill.as_ref().clone();
    self
  }

   
  pub fn animate_outgoing_message_fill(&mut self, animate_outgoing_message_fill: bool) -> &mut Self {
    self.inner.animate_outgoing_message_fill = animate_outgoing_message_fill;
    self
  }

   
  pub fn outgoing_message_accent_color(&mut self, outgoing_message_accent_color: i64) -> &mut Self {
    self.inner.outgoing_message_accent_color = outgoing_message_accent_color;
    self
  }

}

impl AsRef<ThemeSettings> for ThemeSettings {
  fn as_ref(&self) -> &ThemeSettings { self }
}

impl AsRef<ThemeSettings> for RTDThemeSettingsBuilder {
  fn as_ref(&self) -> &ThemeSettings { &self.inner }
}



