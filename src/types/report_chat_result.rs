use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of chat report
pub trait TDReportChatResult: Debug + RObject {}

/// Describes result of chat report
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReportChatResult {
  #[doc(hidden)]
  _Default(()),
  /// Reports a chat to the Telegram moderators. A chat can be reported only from the chat action bar, if chat.can_be_reported, or if messageProperties.can_report_chat
  ReportChat(ReportChat),
  /// The user must choose messages to report and repeat the reportChat request with the chosen messages
  MessagesRequired(ReportChatResultMessagesRequired),
  /// The chat was reported successfully
  Ok(ReportChatResultOk),
  /// The user must choose an option to report the chat and repeat request with the chosen option
  OptionRequired(ReportChatResultOptionRequired),
  /// The user must add additional text details to the report
  TextRequired(ReportChatResultTextRequired),
}

impl Default for ReportChatResult {
  fn default() -> Self {
    ReportChatResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for ReportChatResult {
  fn deserialize<D>(deserializer: D) -> Result<ReportChatResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ReportChatResult,
      (reportChat, ReportChat);
      (reportChatResultMessagesRequired, MessagesRequired);
      (reportChatResultOk, Ok);
      (reportChatResultOptionRequired, OptionRequired);
      (reportChatResultTextRequired, TextRequired);

    )(deserializer)
  }
}

impl RObject for ReportChatResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ReportChatResult::ReportChat(t) => t.td_name(),
      ReportChatResult::MessagesRequired(t) => t.td_name(),
      ReportChatResult::Ok(t) => t.td_name(),
      ReportChatResult::OptionRequired(t) => t.td_name(),
      ReportChatResult::TextRequired(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ReportChatResult::ReportChat(t) => t.extra(),
      ReportChatResult::MessagesRequired(t) => t.extra(),
      ReportChatResult::Ok(t) => t.extra(),
      ReportChatResult::OptionRequired(t) => t.extra(),
      ReportChatResult::TextRequired(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReportChatResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ReportChatResult::_Default(_))
  }

  pub fn is_report_chat(&self) -> bool {
    matches!(self, ReportChatResult::ReportChat(_))
  }
  pub fn is_messages_required(&self) -> bool {
    matches!(self, ReportChatResult::MessagesRequired(_))
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, ReportChatResult::Ok(_))
  }
  pub fn is_option_required(&self) -> bool {
    matches!(self, ReportChatResult::OptionRequired(_))
  }
  pub fn is_text_required(&self) -> bool {
    matches!(self, ReportChatResult::TextRequired(_))
  }

  pub fn on_report_chat<F: FnOnce(&ReportChat)>(&self, fnc: F) -> &Self {
    if let ReportChatResult::ReportChat(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_messages_required<F: FnOnce(&ReportChatResultMessagesRequired)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let ReportChatResult::MessagesRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ok<F: FnOnce(&ReportChatResultOk)>(&self, fnc: F) -> &Self {
    if let ReportChatResult::Ok(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_option_required<F: FnOnce(&ReportChatResultOptionRequired)>(&self, fnc: F) -> &Self {
    if let ReportChatResult::OptionRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text_required<F: FnOnce(&ReportChatResultTextRequired)>(&self, fnc: F) -> &Self {
    if let ReportChatResult::TextRequired(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_report_chat(&self) -> Option<&ReportChat> {
    if let ReportChatResult::ReportChat(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_messages_required(&self) -> Option<&ReportChatResultMessagesRequired> {
    if let ReportChatResult::MessagesRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ok(&self) -> Option<&ReportChatResultOk> {
    if let ReportChatResult::Ok(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_option_required(&self) -> Option<&ReportChatResultOptionRequired> {
    if let ReportChatResult::OptionRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text_required(&self) -> Option<&ReportChatResultTextRequired> {
    if let ReportChatResult::TextRequired(t) = self {
      return Some(t);
    }
    None
  }

  pub fn report_chat<T: AsRef<ReportChat>>(t: T) -> Self {
    ReportChatResult::ReportChat(t.as_ref().clone())
  }

  pub fn messages_required<T: AsRef<ReportChatResultMessagesRequired>>(t: T) -> Self {
    ReportChatResult::MessagesRequired(t.as_ref().clone())
  }

  pub fn ok<T: AsRef<ReportChatResultOk>>(t: T) -> Self {
    ReportChatResult::Ok(t.as_ref().clone())
  }

  pub fn option_required<T: AsRef<ReportChatResultOptionRequired>>(t: T) -> Self {
    ReportChatResult::OptionRequired(t.as_ref().clone())
  }

  pub fn text_required<T: AsRef<ReportChatResultTextRequired>>(t: T) -> Self {
    ReportChatResult::TextRequired(t.as_ref().clone())
  }
}

impl AsRef<ReportChatResult> for ReportChatResult {
  fn as_ref(&self) -> &ReportChatResult {
    self
  }
}

/// The user must choose messages to report and repeat the reportChat request with the chosen messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportChatResultMessagesRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportChatResultMessagesRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportChatResultMessagesRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportChatResult for ReportChatResultMessagesRequired {}

impl ReportChatResultMessagesRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportChatResultMessagesRequiredBuilder {
    let inner = ReportChatResultMessagesRequired {
      td_name: "reportChatResultMessagesRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportChatResultMessagesRequiredBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportChatResultMessagesRequiredBuilder {
  inner: ReportChatResultMessagesRequired,
}

impl RTDReportChatResultMessagesRequiredBuilder {
  pub fn build(&self) -> ReportChatResultMessagesRequired {
    self.inner.clone()
  }
}

impl AsRef<ReportChatResultMessagesRequired> for ReportChatResultMessagesRequired {
  fn as_ref(&self) -> &ReportChatResultMessagesRequired {
    self
  }
}

impl AsRef<ReportChatResultMessagesRequired> for RTDReportChatResultMessagesRequiredBuilder {
  fn as_ref(&self) -> &ReportChatResultMessagesRequired {
    &self.inner
  }
}

/// The chat was reported successfully
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportChatResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportChatResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportChatResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportChatResult for ReportChatResultOk {}

impl ReportChatResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportChatResultOkBuilder {
    let inner = ReportChatResultOk {
      td_name: "reportChatResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportChatResultOkBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportChatResultOkBuilder {
  inner: ReportChatResultOk,
}

impl RTDReportChatResultOkBuilder {
  pub fn build(&self) -> ReportChatResultOk {
    self.inner.clone()
  }
}

impl AsRef<ReportChatResultOk> for ReportChatResultOk {
  fn as_ref(&self) -> &ReportChatResultOk {
    self
  }
}

impl AsRef<ReportChatResultOk> for RTDReportChatResultOkBuilder {
  fn as_ref(&self) -> &ReportChatResultOk {
    &self.inner
  }
}

/// The user must choose an option to report the chat and repeat request with the chosen option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportChatResultOptionRequired {
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

impl RObject for ReportChatResultOptionRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportChatResultOptionRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportChatResult for ReportChatResultOptionRequired {}

impl ReportChatResultOptionRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportChatResultOptionRequiredBuilder {
    let inner = ReportChatResultOptionRequired {
      td_name: "reportChatResultOptionRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReportChatResultOptionRequiredBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn options(&self) -> &Vec<ReportOption> {
    &self.options
  }
}

#[doc(hidden)]
pub struct RTDReportChatResultOptionRequiredBuilder {
  inner: ReportChatResultOptionRequired,
}

impl RTDReportChatResultOptionRequiredBuilder {
  pub fn build(&self) -> ReportChatResultOptionRequired {
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

impl AsRef<ReportChatResultOptionRequired> for ReportChatResultOptionRequired {
  fn as_ref(&self) -> &ReportChatResultOptionRequired {
    self
  }
}

impl AsRef<ReportChatResultOptionRequired> for RTDReportChatResultOptionRequiredBuilder {
  fn as_ref(&self) -> &ReportChatResultOptionRequired {
    &self.inner
  }
}

/// The user must add additional text details to the report
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportChatResultTextRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Option identifier for the next reportChat request
  option_id: String,
  /// True, if the user can skip text adding
  is_optional: bool,
}

impl RObject for ReportChatResultTextRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportChatResultTextRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportChatResult for ReportChatResultTextRequired {}

impl ReportChatResultTextRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportChatResultTextRequiredBuilder {
    let inner = ReportChatResultTextRequired {
      td_name: "reportChatResultTextRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReportChatResultTextRequiredBuilder { inner }
  }

  pub fn option_id(&self) -> &String {
    &self.option_id
  }

  pub fn is_optional(&self) -> bool {
    self.is_optional
  }
}

#[doc(hidden)]
pub struct RTDReportChatResultTextRequiredBuilder {
  inner: ReportChatResultTextRequired,
}

impl RTDReportChatResultTextRequiredBuilder {
  pub fn build(&self) -> ReportChatResultTextRequired {
    self.inner.clone()
  }

  pub fn option_id<T: AsRef<str>>(&mut self, option_id: T) -> &mut Self {
    self.inner.option_id = option_id.as_ref().to_string();
    self
  }

  pub fn is_optional(&mut self, is_optional: bool) -> &mut Self {
    self.inner.is_optional = is_optional;
    self
  }
}

impl AsRef<ReportChatResultTextRequired> for ReportChatResultTextRequired {
  fn as_ref(&self) -> &ReportChatResultTextRequired {
    self
  }
}

impl AsRef<ReportChatResultTextRequired> for RTDReportChatResultTextRequiredBuilder {
  fn as_ref(&self) -> &ReportChatResultTextRequired {
    &self.inner
  }
}
