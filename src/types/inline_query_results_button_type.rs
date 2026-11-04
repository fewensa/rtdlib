use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Represents type of button in results of inline query
pub trait TDInlineQueryResultsButtonType: Debug + RObject {}

/// Represents type of button in results of inline query
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InlineQueryResultsButtonType {
  #[doc(hidden)]
  _Default(()),
  /// Describes the button that opens a private chat with the bot and sends a start message to the bot with the given parameter
  StartBot(InlineQueryResultsButtonTypeStartBot),
  /// Describes the button that opens a Web App by calling getWebAppUrl
  WebApp(InlineQueryResultsButtonTypeWebApp),
}

impl Default for InlineQueryResultsButtonType {
  fn default() -> Self {
    InlineQueryResultsButtonType::_Default(())
  }
}

impl<'de> Deserialize<'de> for InlineQueryResultsButtonType {
  fn deserialize<D>(deserializer: D) -> Result<InlineQueryResultsButtonType, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      InlineQueryResultsButtonType,
      (inlineQueryResultsButtonTypeStartBot, StartBot);
      (inlineQueryResultsButtonTypeWebApp, WebApp);

    )(deserializer)
  }
}

impl RObject for InlineQueryResultsButtonType {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      InlineQueryResultsButtonType::StartBot(t) => t.td_name(),
      InlineQueryResultsButtonType::WebApp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      InlineQueryResultsButtonType::StartBot(t) => t.extra(),
      InlineQueryResultsButtonType::WebApp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InlineQueryResultsButtonType {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, InlineQueryResultsButtonType::_Default(_))
  }

  pub fn is_start_bot(&self) -> bool {
    matches!(self, InlineQueryResultsButtonType::StartBot(_))
  }
  pub fn is_web_app(&self) -> bool {
    matches!(self, InlineQueryResultsButtonType::WebApp(_))
  }

  pub fn on_start_bot<F: FnOnce(&InlineQueryResultsButtonTypeStartBot)>(&self, fnc: F) -> &Self {
    if let InlineQueryResultsButtonType::StartBot(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_web_app<F: FnOnce(&InlineQueryResultsButtonTypeWebApp)>(&self, fnc: F) -> &Self {
    if let InlineQueryResultsButtonType::WebApp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_start_bot(&self) -> Option<&InlineQueryResultsButtonTypeStartBot> {
    if let InlineQueryResultsButtonType::StartBot(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_web_app(&self) -> Option<&InlineQueryResultsButtonTypeWebApp> {
    if let InlineQueryResultsButtonType::WebApp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn start_bot<T: AsRef<InlineQueryResultsButtonTypeStartBot>>(t: T) -> Self {
    InlineQueryResultsButtonType::StartBot(t.as_ref().clone())
  }

  pub fn web_app<T: AsRef<InlineQueryResultsButtonTypeWebApp>>(t: T) -> Self {
    InlineQueryResultsButtonType::WebApp(t.as_ref().clone())
  }
}

impl AsRef<InlineQueryResultsButtonType> for InlineQueryResultsButtonType {
  fn as_ref(&self) -> &InlineQueryResultsButtonType {
    self
  }
}

/// Describes the button that opens a private chat with the bot and sends a start message to the bot with the given parameter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultsButtonTypeStartBot {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The parameter for the bot start message
  parameter: String,
}

impl RObject for InlineQueryResultsButtonTypeStartBot {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineQueryResultsButtonTypeStartBot"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineQueryResultsButtonType for InlineQueryResultsButtonTypeStartBot {}

impl InlineQueryResultsButtonTypeStartBot {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineQueryResultsButtonTypeStartBotBuilder {
    let inner = InlineQueryResultsButtonTypeStartBot {
      td_name: "inlineQueryResultsButtonTypeStartBot".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineQueryResultsButtonTypeStartBotBuilder { inner }
  }

  pub fn parameter(&self) -> &String {
    &self.parameter
  }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultsButtonTypeStartBotBuilder {
  inner: InlineQueryResultsButtonTypeStartBot,
}

impl RTDInlineQueryResultsButtonTypeStartBotBuilder {
  pub fn build(&self) -> InlineQueryResultsButtonTypeStartBot {
    self.inner.clone()
  }

  pub fn parameter<T: AsRef<str>>(&mut self, parameter: T) -> &mut Self {
    self.inner.parameter = parameter.as_ref().to_string();
    self
  }
}

impl AsRef<InlineQueryResultsButtonTypeStartBot> for InlineQueryResultsButtonTypeStartBot {
  fn as_ref(&self) -> &InlineQueryResultsButtonTypeStartBot {
    self
  }
}

impl AsRef<InlineQueryResultsButtonTypeStartBot>
  for RTDInlineQueryResultsButtonTypeStartBotBuilder
{
  fn as_ref(&self) -> &InlineQueryResultsButtonTypeStartBot {
    &self.inner
  }
}

/// Describes the button that opens a Web App by calling getWebAppUrl
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultsButtonTypeWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// An HTTP URL to pass to getWebAppUrl
  url: String,
}

impl RObject for InlineQueryResultsButtonTypeWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inlineQueryResultsButtonTypeWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDInlineQueryResultsButtonType for InlineQueryResultsButtonTypeWebApp {}

impl InlineQueryResultsButtonTypeWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInlineQueryResultsButtonTypeWebAppBuilder {
    let inner = InlineQueryResultsButtonTypeWebApp {
      td_name: "inlineQueryResultsButtonTypeWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInlineQueryResultsButtonTypeWebAppBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDInlineQueryResultsButtonTypeWebAppBuilder {
  inner: InlineQueryResultsButtonTypeWebApp,
}

impl RTDInlineQueryResultsButtonTypeWebAppBuilder {
  pub fn build(&self) -> InlineQueryResultsButtonTypeWebApp {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<InlineQueryResultsButtonTypeWebApp> for InlineQueryResultsButtonTypeWebApp {
  fn as_ref(&self) -> &InlineQueryResultsButtonTypeWebApp {
    self
  }
}

impl AsRef<InlineQueryResultsButtonTypeWebApp> for RTDInlineQueryResultsButtonTypeWebAppBuilder {
  fn as_ref(&self) -> &InlineQueryResultsButtonTypeWebApp {
    &self.inner
  }
}
