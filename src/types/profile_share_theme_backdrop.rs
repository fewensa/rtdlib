use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a backdrop, which can be used as a background of a theme of the screen for sharing a link to a user or a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileShareThemeBackdrop {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the backdrop
  id: i64,
  /// Name of the backdrop
  name: String,
  /// Colors of the backdrop
  colors: UpgradedGiftBackdropColors,
}

impl RObject for ProfileShareThemeBackdrop {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileShareThemeBackdrop"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ProfileShareThemeBackdrop {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileShareThemeBackdropBuilder {
    let inner = ProfileShareThemeBackdrop {
      td_name: "profileShareThemeBackdrop".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDProfileShareThemeBackdropBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn colors(&self) -> &UpgradedGiftBackdropColors {
    &self.colors
  }
}

#[doc(hidden)]
pub struct RTDProfileShareThemeBackdropBuilder {
  inner: ProfileShareThemeBackdrop,
}

impl RTDProfileShareThemeBackdropBuilder {
  pub fn build(&self) -> ProfileShareThemeBackdrop {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn colors<T: AsRef<UpgradedGiftBackdropColors>>(&mut self, colors: T) -> &mut Self {
    self.inner.colors = colors.as_ref().clone();
    self
  }
}

impl AsRef<ProfileShareThemeBackdrop> for ProfileShareThemeBackdrop {
  fn as_ref(&self) -> &ProfileShareThemeBackdrop {
    self
  }
}

impl AsRef<ProfileShareThemeBackdrop> for RTDProfileShareThemeBackdropBuilder {
  fn as_ref(&self) -> &ProfileShareThemeBackdrop {
    &self.inner
  }
}
