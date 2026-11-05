use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a background set for a specific chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatBackground {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The background
  background: Background,
  /// Dimming of the background in dark themes, as a percentage; 0-100. Applied only to Wallpaper and Fill types of background
  dark_theme_dimming: i64,
}

impl RObject for ChatBackground {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatBackground"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatBackground {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatBackgroundBuilder {
    let inner = ChatBackground {
      td_name: "chatBackground".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatBackgroundBuilder { inner }
  }

  pub fn background(&self) -> &Background {
    &self.background
  }

  pub fn dark_theme_dimming(&self) -> i64 {
    self.dark_theme_dimming
  }
}

#[doc(hidden)]
pub struct RTDChatBackgroundBuilder {
  inner: ChatBackground,
}

impl RTDChatBackgroundBuilder {
  pub fn build(&self) -> ChatBackground {
    self.inner.clone()
  }

  pub fn background<T: AsRef<Background>>(&mut self, background: T) -> &mut Self {
    self.inner.background = background.as_ref().clone();
    self
  }

  pub fn dark_theme_dimming(&mut self, dark_theme_dimming: i64) -> &mut Self {
    self.inner.dark_theme_dimming = dark_theme_dimming;
    self
  }
}

impl AsRef<ChatBackground> for ChatBackground {
  fn as_ref(&self) -> &ChatBackground {
    self
  }
}

impl AsRef<ChatBackground> for RTDChatBackgroundBuilder {
  fn as_ref(&self) -> &ChatBackground {
    &self.inner
  }
}
