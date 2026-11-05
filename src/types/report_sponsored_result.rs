use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of sponsored message or chat report
pub trait TDReportSponsoredResult: Debug + RObject {}

/// Describes result of sponsored message or chat report
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReportSponsoredResult {
  #[doc(hidden)]
  _Default(()),
  /// Reports a sponsored message to Telegram moderators
  ReportChatSponsoredMessage(ReportChatSponsoredMessage),
  /// Reports a sponsored chat to Telegram moderators
  ReportSponsoredChat(ReportSponsoredChat),
  /// Sponsored messages were hidden for the user in all chats
  AdsHidden(ReportSponsoredResultAdsHidden),
  /// The sponsored message is too old or not found
  Failed(ReportSponsoredResultFailed),
  /// The message was reported successfully
  Ok(ReportSponsoredResultOk),
  /// The user must choose an option to report the message and repeat request with the chosen option
  OptionRequired(ReportSponsoredResultOptionRequired),
  /// The user asked to hide sponsored messages, but Telegram Premium is required for this
  PremiumRequired(ReportSponsoredResultPremiumRequired),
  /// Reports a video message advertisement to Telegram moderators
  ReportVideoMessageAdvertisement(ReportVideoMessageAdvertisement),
}

impl Default for ReportSponsoredResult {
  fn default() -> Self {
    ReportSponsoredResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for ReportSponsoredResult {
  fn deserialize<D>(deserializer: D) -> Result<ReportSponsoredResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ReportSponsoredResult,
      (reportChatSponsoredMessage, ReportChatSponsoredMessage);
      (reportSponsoredChat, ReportSponsoredChat);
      (reportSponsoredResultAdsHidden, AdsHidden);
      (reportSponsoredResultFailed, Failed);
      (reportSponsoredResultOk, Ok);
      (reportSponsoredResultOptionRequired, OptionRequired);
      (reportSponsoredResultPremiumRequired, PremiumRequired);
      (reportVideoMessageAdvertisement, ReportVideoMessageAdvertisement);

    )(deserializer)
  }
}

impl RObject for ReportSponsoredResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ReportSponsoredResult::ReportChatSponsoredMessage(t) => t.td_name(),
      ReportSponsoredResult::ReportSponsoredChat(t) => t.td_name(),
      ReportSponsoredResult::AdsHidden(t) => t.td_name(),
      ReportSponsoredResult::Failed(t) => t.td_name(),
      ReportSponsoredResult::Ok(t) => t.td_name(),
      ReportSponsoredResult::OptionRequired(t) => t.td_name(),
      ReportSponsoredResult::PremiumRequired(t) => t.td_name(),
      ReportSponsoredResult::ReportVideoMessageAdvertisement(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ReportSponsoredResult::ReportChatSponsoredMessage(t) => t.extra(),
      ReportSponsoredResult::ReportSponsoredChat(t) => t.extra(),
      ReportSponsoredResult::AdsHidden(t) => t.extra(),
      ReportSponsoredResult::Failed(t) => t.extra(),
      ReportSponsoredResult::Ok(t) => t.extra(),
      ReportSponsoredResult::OptionRequired(t) => t.extra(),
      ReportSponsoredResult::PremiumRequired(t) => t.extra(),
      ReportSponsoredResult::ReportVideoMessageAdvertisement(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReportSponsoredResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ReportSponsoredResult::_Default(_))
  }

  pub fn is_report_chat_sponsored_message(&self) -> bool {
    matches!(self, ReportSponsoredResult::ReportChatSponsoredMessage(_))
  }
  pub fn is_report_sponsored_chat(&self) -> bool {
    matches!(self, ReportSponsoredResult::ReportSponsoredChat(_))
  }
  pub fn is_ads_hidden(&self) -> bool {
    matches!(self, ReportSponsoredResult::AdsHidden(_))
  }
  pub fn is_failed(&self) -> bool {
    matches!(self, ReportSponsoredResult::Failed(_))
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, ReportSponsoredResult::Ok(_))
  }
  pub fn is_option_required(&self) -> bool {
    matches!(self, ReportSponsoredResult::OptionRequired(_))
  }
  pub fn is_premium_required(&self) -> bool {
    matches!(self, ReportSponsoredResult::PremiumRequired(_))
  }
  pub fn is_report_video_message_advertisement(&self) -> bool {
    matches!(
      self,
      ReportSponsoredResult::ReportVideoMessageAdvertisement(_)
    )
  }

  pub fn on_report_chat_sponsored_message<F: FnOnce(&ReportChatSponsoredMessage)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ReportSponsoredResult::ReportChatSponsoredMessage(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_report_sponsored_chat<F: FnOnce(&ReportSponsoredChat)>(&self, fnc: F) -> &Self {
    if let ReportSponsoredResult::ReportSponsoredChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ads_hidden<F: FnOnce(&ReportSponsoredResultAdsHidden)>(&self, fnc: F) -> &Self {
    if let ReportSponsoredResult::AdsHidden(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_failed<F: FnOnce(&ReportSponsoredResultFailed)>(&self, fnc: F) -> &Self {
    if let ReportSponsoredResult::Failed(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ok<F: FnOnce(&ReportSponsoredResultOk)>(&self, fnc: F) -> &Self {
    if let ReportSponsoredResult::Ok(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_option_required<F: FnOnce(&ReportSponsoredResultOptionRequired)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ReportSponsoredResult::OptionRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_premium_required<F: FnOnce(&ReportSponsoredResultPremiumRequired)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ReportSponsoredResult::PremiumRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_report_video_message_advertisement<F: FnOnce(&ReportVideoMessageAdvertisement)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ReportSponsoredResult::ReportVideoMessageAdvertisement(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_report_chat_sponsored_message(&self) -> Option<&ReportChatSponsoredMessage> {
    if let ReportSponsoredResult::ReportChatSponsoredMessage(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_report_sponsored_chat(&self) -> Option<&ReportSponsoredChat> {
    if let ReportSponsoredResult::ReportSponsoredChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ads_hidden(&self) -> Option<&ReportSponsoredResultAdsHidden> {
    if let ReportSponsoredResult::AdsHidden(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_failed(&self) -> Option<&ReportSponsoredResultFailed> {
    if let ReportSponsoredResult::Failed(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ok(&self) -> Option<&ReportSponsoredResultOk> {
    if let ReportSponsoredResult::Ok(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_option_required(&self) -> Option<&ReportSponsoredResultOptionRequired> {
    if let ReportSponsoredResult::OptionRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_premium_required(&self) -> Option<&ReportSponsoredResultPremiumRequired> {
    if let ReportSponsoredResult::PremiumRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_report_video_message_advertisement(&self) -> Option<&ReportVideoMessageAdvertisement> {
    if let ReportSponsoredResult::ReportVideoMessageAdvertisement(t) = self {
      return Some(t);
    }
    None
  }

  pub fn report_chat_sponsored_message<T: AsRef<ReportChatSponsoredMessage>>(t: T) -> Self {
    ReportSponsoredResult::ReportChatSponsoredMessage(t.as_ref().clone())
  }

  pub fn report_sponsored_chat<T: AsRef<ReportSponsoredChat>>(t: T) -> Self {
    ReportSponsoredResult::ReportSponsoredChat(t.as_ref().clone())
  }

  pub fn ads_hidden<T: AsRef<ReportSponsoredResultAdsHidden>>(t: T) -> Self {
    ReportSponsoredResult::AdsHidden(t.as_ref().clone())
  }

  pub fn failed<T: AsRef<ReportSponsoredResultFailed>>(t: T) -> Self {
    ReportSponsoredResult::Failed(t.as_ref().clone())
  }

  pub fn ok<T: AsRef<ReportSponsoredResultOk>>(t: T) -> Self {
    ReportSponsoredResult::Ok(t.as_ref().clone())
  }

  pub fn option_required<T: AsRef<ReportSponsoredResultOptionRequired>>(t: T) -> Self {
    ReportSponsoredResult::OptionRequired(t.as_ref().clone())
  }

  pub fn premium_required<T: AsRef<ReportSponsoredResultPremiumRequired>>(t: T) -> Self {
    ReportSponsoredResult::PremiumRequired(t.as_ref().clone())
  }

  pub fn report_video_message_advertisement<T: AsRef<ReportVideoMessageAdvertisement>>(
    t: T,
  ) -> Self {
    ReportSponsoredResult::ReportVideoMessageAdvertisement(t.as_ref().clone())
  }
}

impl AsRef<ReportSponsoredResult> for ReportSponsoredResult {
  fn as_ref(&self) -> &ReportSponsoredResult {
    self
  }
}

/// Sponsored messages were hidden for the user in all chats
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSponsoredResultAdsHidden {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportSponsoredResultAdsHidden {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportSponsoredResultAdsHidden"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportSponsoredResult for ReportSponsoredResultAdsHidden {}

impl ReportSponsoredResultAdsHidden {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportSponsoredResultAdsHiddenBuilder {
    let inner = ReportSponsoredResultAdsHidden {
      td_name: "reportSponsoredResultAdsHidden".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportSponsoredResultAdsHiddenBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportSponsoredResultAdsHiddenBuilder {
  inner: ReportSponsoredResultAdsHidden,
}

impl RTDReportSponsoredResultAdsHiddenBuilder {
  pub fn build(&self) -> ReportSponsoredResultAdsHidden {
    self.inner.clone()
  }
}

impl AsRef<ReportSponsoredResultAdsHidden> for ReportSponsoredResultAdsHidden {
  fn as_ref(&self) -> &ReportSponsoredResultAdsHidden {
    self
  }
}

impl AsRef<ReportSponsoredResultAdsHidden> for RTDReportSponsoredResultAdsHiddenBuilder {
  fn as_ref(&self) -> &ReportSponsoredResultAdsHidden {
    &self.inner
  }
}

/// The sponsored message is too old or not found
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSponsoredResultFailed {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportSponsoredResultFailed {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportSponsoredResultFailed"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportSponsoredResult for ReportSponsoredResultFailed {}

impl ReportSponsoredResultFailed {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportSponsoredResultFailedBuilder {
    let inner = ReportSponsoredResultFailed {
      td_name: "reportSponsoredResultFailed".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportSponsoredResultFailedBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportSponsoredResultFailedBuilder {
  inner: ReportSponsoredResultFailed,
}

impl RTDReportSponsoredResultFailedBuilder {
  pub fn build(&self) -> ReportSponsoredResultFailed {
    self.inner.clone()
  }
}

impl AsRef<ReportSponsoredResultFailed> for ReportSponsoredResultFailed {
  fn as_ref(&self) -> &ReportSponsoredResultFailed {
    self
  }
}

impl AsRef<ReportSponsoredResultFailed> for RTDReportSponsoredResultFailedBuilder {
  fn as_ref(&self) -> &ReportSponsoredResultFailed {
    &self.inner
  }
}

/// The message was reported successfully
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSponsoredResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportSponsoredResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportSponsoredResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportSponsoredResult for ReportSponsoredResultOk {}

impl ReportSponsoredResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportSponsoredResultOkBuilder {
    let inner = ReportSponsoredResultOk {
      td_name: "reportSponsoredResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportSponsoredResultOkBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportSponsoredResultOkBuilder {
  inner: ReportSponsoredResultOk,
}

impl RTDReportSponsoredResultOkBuilder {
  pub fn build(&self) -> ReportSponsoredResultOk {
    self.inner.clone()
  }
}

impl AsRef<ReportSponsoredResultOk> for ReportSponsoredResultOk {
  fn as_ref(&self) -> &ReportSponsoredResultOk {
    self
  }
}

impl AsRef<ReportSponsoredResultOk> for RTDReportSponsoredResultOkBuilder {
  fn as_ref(&self) -> &ReportSponsoredResultOk {
    &self.inner
  }
}

/// The user must choose an option to report the message and repeat request with the chosen option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSponsoredResultOptionRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Title for the option choice
  title: String,
  /// List of available options
  options: Vec<ReportOption>,
}

impl RObject for ReportSponsoredResultOptionRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportSponsoredResultOptionRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportSponsoredResult for ReportSponsoredResultOptionRequired {}

impl ReportSponsoredResultOptionRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportSponsoredResultOptionRequiredBuilder {
    let inner = ReportSponsoredResultOptionRequired {
      td_name: "reportSponsoredResultOptionRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReportSponsoredResultOptionRequiredBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn options(&self) -> &Vec<ReportOption> {
    &self.options
  }
}

#[doc(hidden)]
pub struct RTDReportSponsoredResultOptionRequiredBuilder {
  inner: ReportSponsoredResultOptionRequired,
}

impl RTDReportSponsoredResultOptionRequiredBuilder {
  pub fn build(&self) -> ReportSponsoredResultOptionRequired {
    self.inner.clone()
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn options(&mut self, options: Vec<ReportOption>) -> &mut Self {
    self.inner.options = options;
    self
  }
}

impl AsRef<ReportSponsoredResultOptionRequired> for ReportSponsoredResultOptionRequired {
  fn as_ref(&self) -> &ReportSponsoredResultOptionRequired {
    self
  }
}

impl AsRef<ReportSponsoredResultOptionRequired> for RTDReportSponsoredResultOptionRequiredBuilder {
  fn as_ref(&self) -> &ReportSponsoredResultOptionRequired {
    &self.inner
  }
}

/// The user asked to hide sponsored messages, but Telegram Premium is required for this
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportSponsoredResultPremiumRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportSponsoredResultPremiumRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportSponsoredResultPremiumRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportSponsoredResult for ReportSponsoredResultPremiumRequired {}

impl ReportSponsoredResultPremiumRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportSponsoredResultPremiumRequiredBuilder {
    let inner = ReportSponsoredResultPremiumRequired {
      td_name: "reportSponsoredResultPremiumRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportSponsoredResultPremiumRequiredBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportSponsoredResultPremiumRequiredBuilder {
  inner: ReportSponsoredResultPremiumRequired,
}

impl RTDReportSponsoredResultPremiumRequiredBuilder {
  pub fn build(&self) -> ReportSponsoredResultPremiumRequired {
    self.inner.clone()
  }
}

impl AsRef<ReportSponsoredResultPremiumRequired> for ReportSponsoredResultPremiumRequired {
  fn as_ref(&self) -> &ReportSponsoredResultPremiumRequired {
    self
  }
}

impl AsRef<ReportSponsoredResultPremiumRequired>
  for RTDReportSponsoredResultPremiumRequiredBuilder
{
  fn as_ref(&self) -> &ReportSponsoredResultPremiumRequired {
    &self.inner
  }
}
