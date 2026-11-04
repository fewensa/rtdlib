use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a button to be shown instead of bot commands menu button
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotMenuButton {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Text of the button
  text: String,
  /// URL of a Web App to open when the button is pressed. If the link is of the type internalLinkTypeWebApp, then it must be processed accordingly. Otherwise, the link must be passed to openWebApp
  url: String,
}

impl RObject for BotMenuButton {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botMenuButton"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotMenuButton {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotMenuButtonBuilder {
    let inner = BotMenuButton {
      td_name: "botMenuButton".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotMenuButtonBuilder { inner }
  }

  pub fn text(&self) -> &String {
    &self.text
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDBotMenuButtonBuilder {
  inner: BotMenuButton,
}

impl RTDBotMenuButtonBuilder {
  pub fn build(&self) -> BotMenuButton {
    self.inner.clone()
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<BotMenuButton> for BotMenuButton {
  fn as_ref(&self) -> &BotMenuButton {
    self
  }
}

impl AsRef<BotMenuButton> for RTDBotMenuButtonBuilder {
  fn as_ref(&self) -> &BotMenuButton {
    &self.inner
  }
}
