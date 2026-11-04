use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a reason why a bot was allowed to write messages to the current user
pub trait TDBotWriteAccessAllowReason: Debug + RObject {}

/// Describes a reason why a bot was allowed to write messages to the current user
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum BotWriteAccessAllowReason {
  #[doc(hidden)]
  _Default(()),
  /// The user accepted bot's request to send messages with allowBotToSendMessages
  AcceptedRequest(BotWriteAccessAllowReasonAcceptedRequest),
  /// The user added the bot to attachment or side menu using toggleBotIsAddedToAttachmentMenu
  AddedToAttachmentMenu(BotWriteAccessAllowReasonAddedToAttachmentMenu),
  /// The user connected a website by logging in using Telegram Login Widget on it
  ConnectedWebsite(BotWriteAccessAllowReasonConnectedWebsite),
  /// The user launched a Web App using getWebAppLinkUrl
  LaunchedWebApp(BotWriteAccessAllowReasonLaunchedWebApp),
}

impl Default for BotWriteAccessAllowReason {
  fn default() -> Self {
    BotWriteAccessAllowReason::_Default(())
  }
}

impl<'de> Deserialize<'de> for BotWriteAccessAllowReason {
  fn deserialize<D>(deserializer: D) -> Result<BotWriteAccessAllowReason, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      BotWriteAccessAllowReason,
      (botWriteAccessAllowReasonAcceptedRequest, AcceptedRequest);
      (botWriteAccessAllowReasonAddedToAttachmentMenu, AddedToAttachmentMenu);
      (botWriteAccessAllowReasonConnectedWebsite, ConnectedWebsite);
      (botWriteAccessAllowReasonLaunchedWebApp, LaunchedWebApp);

    )(deserializer)
  }
}

impl RObject for BotWriteAccessAllowReason {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      BotWriteAccessAllowReason::AcceptedRequest(t) => t.td_name(),
      BotWriteAccessAllowReason::AddedToAttachmentMenu(t) => t.td_name(),
      BotWriteAccessAllowReason::ConnectedWebsite(t) => t.td_name(),
      BotWriteAccessAllowReason::LaunchedWebApp(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      BotWriteAccessAllowReason::AcceptedRequest(t) => t.extra(),
      BotWriteAccessAllowReason::AddedToAttachmentMenu(t) => t.extra(),
      BotWriteAccessAllowReason::ConnectedWebsite(t) => t.extra(),
      BotWriteAccessAllowReason::LaunchedWebApp(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl BotWriteAccessAllowReason {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, BotWriteAccessAllowReason::_Default(_))
  }

  pub fn is_accepted_request(&self) -> bool {
    matches!(self, BotWriteAccessAllowReason::AcceptedRequest(_))
  }
  pub fn is_added_to_attachment_menu(&self) -> bool {
    matches!(self, BotWriteAccessAllowReason::AddedToAttachmentMenu(_))
  }
  pub fn is_connected_website(&self) -> bool {
    matches!(self, BotWriteAccessAllowReason::ConnectedWebsite(_))
  }
  pub fn is_launched_web_app(&self) -> bool {
    matches!(self, BotWriteAccessAllowReason::LaunchedWebApp(_))
  }

  pub fn on_accepted_request<F: FnOnce(&BotWriteAccessAllowReasonAcceptedRequest)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let BotWriteAccessAllowReason::AcceptedRequest(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_added_to_attachment_menu<F: FnOnce(&BotWriteAccessAllowReasonAddedToAttachmentMenu)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let BotWriteAccessAllowReason::AddedToAttachmentMenu(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_connected_website<F: FnOnce(&BotWriteAccessAllowReasonConnectedWebsite)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let BotWriteAccessAllowReason::ConnectedWebsite(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_launched_web_app<F: FnOnce(&BotWriteAccessAllowReasonLaunchedWebApp)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let BotWriteAccessAllowReason::LaunchedWebApp(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_accepted_request(&self) -> Option<&BotWriteAccessAllowReasonAcceptedRequest> {
    if let BotWriteAccessAllowReason::AcceptedRequest(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_added_to_attachment_menu(
    &self,
  ) -> Option<&BotWriteAccessAllowReasonAddedToAttachmentMenu> {
    if let BotWriteAccessAllowReason::AddedToAttachmentMenu(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_connected_website(&self) -> Option<&BotWriteAccessAllowReasonConnectedWebsite> {
    if let BotWriteAccessAllowReason::ConnectedWebsite(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_launched_web_app(&self) -> Option<&BotWriteAccessAllowReasonLaunchedWebApp> {
    if let BotWriteAccessAllowReason::LaunchedWebApp(t) = self {
      return Some(t);
    }
    None
  }

  pub fn accepted_request<T: AsRef<BotWriteAccessAllowReasonAcceptedRequest>>(t: T) -> Self {
    BotWriteAccessAllowReason::AcceptedRequest(t.as_ref().clone())
  }

  pub fn added_to_attachment_menu<T: AsRef<BotWriteAccessAllowReasonAddedToAttachmentMenu>>(
    t: T,
  ) -> Self {
    BotWriteAccessAllowReason::AddedToAttachmentMenu(t.as_ref().clone())
  }

  pub fn connected_website<T: AsRef<BotWriteAccessAllowReasonConnectedWebsite>>(t: T) -> Self {
    BotWriteAccessAllowReason::ConnectedWebsite(t.as_ref().clone())
  }

  pub fn launched_web_app<T: AsRef<BotWriteAccessAllowReasonLaunchedWebApp>>(t: T) -> Self {
    BotWriteAccessAllowReason::LaunchedWebApp(t.as_ref().clone())
  }
}

impl AsRef<BotWriteAccessAllowReason> for BotWriteAccessAllowReason {
  fn as_ref(&self) -> &BotWriteAccessAllowReason {
    self
  }
}

/// The user accepted bot's request to send messages with allowBotToSendMessages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotWriteAccessAllowReasonAcceptedRequest {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BotWriteAccessAllowReasonAcceptedRequest {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botWriteAccessAllowReasonAcceptedRequest"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBotWriteAccessAllowReason for BotWriteAccessAllowReasonAcceptedRequest {}

impl BotWriteAccessAllowReasonAcceptedRequest {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotWriteAccessAllowReasonAcceptedRequestBuilder {
    let inner = BotWriteAccessAllowReasonAcceptedRequest {
      td_name: "botWriteAccessAllowReasonAcceptedRequest".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBotWriteAccessAllowReasonAcceptedRequestBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBotWriteAccessAllowReasonAcceptedRequestBuilder {
  inner: BotWriteAccessAllowReasonAcceptedRequest,
}

impl RTDBotWriteAccessAllowReasonAcceptedRequestBuilder {
  pub fn build(&self) -> BotWriteAccessAllowReasonAcceptedRequest {
    self.inner.clone()
  }
}

impl AsRef<BotWriteAccessAllowReasonAcceptedRequest> for BotWriteAccessAllowReasonAcceptedRequest {
  fn as_ref(&self) -> &BotWriteAccessAllowReasonAcceptedRequest {
    self
  }
}

impl AsRef<BotWriteAccessAllowReasonAcceptedRequest>
  for RTDBotWriteAccessAllowReasonAcceptedRequestBuilder
{
  fn as_ref(&self) -> &BotWriteAccessAllowReasonAcceptedRequest {
    &self.inner
  }
}

/// The user added the bot to attachment or side menu using toggleBotIsAddedToAttachmentMenu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotWriteAccessAllowReasonAddedToAttachmentMenu {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for BotWriteAccessAllowReasonAddedToAttachmentMenu {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botWriteAccessAllowReasonAddedToAttachmentMenu"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBotWriteAccessAllowReason for BotWriteAccessAllowReasonAddedToAttachmentMenu {}

impl BotWriteAccessAllowReasonAddedToAttachmentMenu {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotWriteAccessAllowReasonAddedToAttachmentMenuBuilder {
    let inner = BotWriteAccessAllowReasonAddedToAttachmentMenu {
      td_name: "botWriteAccessAllowReasonAddedToAttachmentMenu".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDBotWriteAccessAllowReasonAddedToAttachmentMenuBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDBotWriteAccessAllowReasonAddedToAttachmentMenuBuilder {
  inner: BotWriteAccessAllowReasonAddedToAttachmentMenu,
}

impl RTDBotWriteAccessAllowReasonAddedToAttachmentMenuBuilder {
  pub fn build(&self) -> BotWriteAccessAllowReasonAddedToAttachmentMenu {
    self.inner.clone()
  }
}

impl AsRef<BotWriteAccessAllowReasonAddedToAttachmentMenu>
  for BotWriteAccessAllowReasonAddedToAttachmentMenu
{
  fn as_ref(&self) -> &BotWriteAccessAllowReasonAddedToAttachmentMenu {
    self
  }
}

impl AsRef<BotWriteAccessAllowReasonAddedToAttachmentMenu>
  for RTDBotWriteAccessAllowReasonAddedToAttachmentMenuBuilder
{
  fn as_ref(&self) -> &BotWriteAccessAllowReasonAddedToAttachmentMenu {
    &self.inner
  }
}

/// The user connected a website by logging in using Telegram Login Widget on it
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotWriteAccessAllowReasonConnectedWebsite {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Domain name of the connected website
  domain_name: String,
}

impl RObject for BotWriteAccessAllowReasonConnectedWebsite {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botWriteAccessAllowReasonConnectedWebsite"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBotWriteAccessAllowReason for BotWriteAccessAllowReasonConnectedWebsite {}

impl BotWriteAccessAllowReasonConnectedWebsite {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotWriteAccessAllowReasonConnectedWebsiteBuilder {
    let inner = BotWriteAccessAllowReasonConnectedWebsite {
      td_name: "botWriteAccessAllowReasonConnectedWebsite".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotWriteAccessAllowReasonConnectedWebsiteBuilder { inner }
  }

  pub fn domain_name(&self) -> &String {
    &self.domain_name
  }
}

#[doc(hidden)]
pub struct RTDBotWriteAccessAllowReasonConnectedWebsiteBuilder {
  inner: BotWriteAccessAllowReasonConnectedWebsite,
}

impl RTDBotWriteAccessAllowReasonConnectedWebsiteBuilder {
  pub fn build(&self) -> BotWriteAccessAllowReasonConnectedWebsite {
    self.inner.clone()
  }

  pub fn domain_name<T: AsRef<str>>(&mut self, domain_name: T) -> &mut Self {
    self.inner.domain_name = domain_name.as_ref().to_string();
    self
  }
}

impl AsRef<BotWriteAccessAllowReasonConnectedWebsite>
  for BotWriteAccessAllowReasonConnectedWebsite
{
  fn as_ref(&self) -> &BotWriteAccessAllowReasonConnectedWebsite {
    self
  }
}

impl AsRef<BotWriteAccessAllowReasonConnectedWebsite>
  for RTDBotWriteAccessAllowReasonConnectedWebsiteBuilder
{
  fn as_ref(&self) -> &BotWriteAccessAllowReasonConnectedWebsite {
    &self.inner
  }
}

/// The user launched a Web App using getWebAppLinkUrl
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BotWriteAccessAllowReasonLaunchedWebApp {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Information about the Web App
  web_app: WebApp,
}

impl RObject for BotWriteAccessAllowReasonLaunchedWebApp {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "botWriteAccessAllowReasonLaunchedWebApp"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDBotWriteAccessAllowReason for BotWriteAccessAllowReasonLaunchedWebApp {}

impl BotWriteAccessAllowReasonLaunchedWebApp {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDBotWriteAccessAllowReasonLaunchedWebAppBuilder {
    let inner = BotWriteAccessAllowReasonLaunchedWebApp {
      td_name: "botWriteAccessAllowReasonLaunchedWebApp".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDBotWriteAccessAllowReasonLaunchedWebAppBuilder { inner }
  }

  pub fn web_app(&self) -> &WebApp {
    &self.web_app
  }
}

#[doc(hidden)]
pub struct RTDBotWriteAccessAllowReasonLaunchedWebAppBuilder {
  inner: BotWriteAccessAllowReasonLaunchedWebApp,
}

impl RTDBotWriteAccessAllowReasonLaunchedWebAppBuilder {
  pub fn build(&self) -> BotWriteAccessAllowReasonLaunchedWebApp {
    self.inner.clone()
  }

  pub fn web_app<T: AsRef<WebApp>>(&mut self, web_app: T) -> &mut Self {
    self.inner.web_app = web_app.as_ref().clone();
    self
  }
}

impl AsRef<BotWriteAccessAllowReasonLaunchedWebApp> for BotWriteAccessAllowReasonLaunchedWebApp {
  fn as_ref(&self) -> &BotWriteAccessAllowReasonLaunchedWebApp {
    self
  }
}

impl AsRef<BotWriteAccessAllowReasonLaunchedWebApp>
  for RTDBotWriteAccessAllowReasonLaunchedWebAppBuilder
{
  fn as_ref(&self) -> &BotWriteAccessAllowReasonLaunchedWebApp {
    &self.inner
  }
}
