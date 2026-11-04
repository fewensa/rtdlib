use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of backdrops, which can be used as a background of a theme of the screen for sharing a link to a user or a chat
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfileShareThemeBackdrops {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The list of backdrops
  backdrops: Vec<ProfileShareThemeBackdrop>,
}

impl RObject for ProfileShareThemeBackdrops {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "profileShareThemeBackdrops"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ProfileShareThemeBackdrops {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDProfileShareThemeBackdropsBuilder {
    let inner = ProfileShareThemeBackdrops {
      td_name: "profileShareThemeBackdrops".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDProfileShareThemeBackdropsBuilder { inner }
  }

  pub fn backdrops(&self) -> &Vec<ProfileShareThemeBackdrop> {
    &self.backdrops
  }
}

#[doc(hidden)]
pub struct RTDProfileShareThemeBackdropsBuilder {
  inner: ProfileShareThemeBackdrops,
}

impl RTDProfileShareThemeBackdropsBuilder {
  pub fn build(&self) -> ProfileShareThemeBackdrops {
    self.inner.clone()
  }

  pub fn backdrops(&mut self, backdrops: Vec<ProfileShareThemeBackdrop>) -> &mut Self {
    self.inner.backdrops = backdrops;
    self
  }
}

impl AsRef<ProfileShareThemeBackdrops> for ProfileShareThemeBackdrops {
  fn as_ref(&self) -> &ProfileShareThemeBackdrops {
    self
  }
}

impl AsRef<ProfileShareThemeBackdrops> for RTDProfileShareThemeBackdropsBuilder {
  fn as_ref(&self) -> &ProfileShareThemeBackdrops {
    &self.inner
  }
}
