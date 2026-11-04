use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes result of story report
pub trait TDReportStoryResult: Debug + RObject {}

/// Describes result of story report
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReportStoryResult {
  #[doc(hidden)]
  _Default(()),
  /// Reports a story to the Telegram moderators
  ReportStory(ReportStory),
  /// The story was reported successfully
  Ok(ReportStoryResultOk),
  /// The user must choose an option to report the story and repeat request with the chosen option
  OptionRequired(ReportStoryResultOptionRequired),
  /// The user must add additional text details to the report
  TextRequired(ReportStoryResultTextRequired),
}

impl Default for ReportStoryResult {
  fn default() -> Self {
    ReportStoryResult::_Default(())
  }
}

impl<'de> Deserialize<'de> for ReportStoryResult {
  fn deserialize<D>(deserializer: D) -> Result<ReportStoryResult, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ReportStoryResult,
      (reportStory, ReportStory);
      (reportStoryResultOk, Ok);
      (reportStoryResultOptionRequired, OptionRequired);
      (reportStoryResultTextRequired, TextRequired);

    )(deserializer)
  }
}

impl RObject for ReportStoryResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ReportStoryResult::ReportStory(t) => t.td_name(),
      ReportStoryResult::Ok(t) => t.td_name(),
      ReportStoryResult::OptionRequired(t) => t.td_name(),
      ReportStoryResult::TextRequired(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ReportStoryResult::ReportStory(t) => t.extra(),
      ReportStoryResult::Ok(t) => t.extra(),
      ReportStoryResult::OptionRequired(t) => t.extra(),
      ReportStoryResult::TextRequired(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReportStoryResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ReportStoryResult::_Default(_))
  }

  pub fn is_report_story(&self) -> bool {
    matches!(self, ReportStoryResult::ReportStory(_))
  }
  pub fn is_ok(&self) -> bool {
    matches!(self, ReportStoryResult::Ok(_))
  }
  pub fn is_option_required(&self) -> bool {
    matches!(self, ReportStoryResult::OptionRequired(_))
  }
  pub fn is_text_required(&self) -> bool {
    matches!(self, ReportStoryResult::TextRequired(_))
  }

  pub fn on_report_story<F: FnOnce(&ReportStory)>(&self, fnc: F) -> &Self {
    if let ReportStoryResult::ReportStory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_ok<F: FnOnce(&ReportStoryResultOk)>(&self, fnc: F) -> &Self {
    if let ReportStoryResult::Ok(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_option_required<F: FnOnce(&ReportStoryResultOptionRequired)>(&self, fnc: F) -> &Self {
    if let ReportStoryResult::OptionRequired(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_text_required<F: FnOnce(&ReportStoryResultTextRequired)>(&self, fnc: F) -> &Self {
    if let ReportStoryResult::TextRequired(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_report_story(&self) -> Option<&ReportStory> {
    if let ReportStoryResult::ReportStory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_ok(&self) -> Option<&ReportStoryResultOk> {
    if let ReportStoryResult::Ok(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_option_required(&self) -> Option<&ReportStoryResultOptionRequired> {
    if let ReportStoryResult::OptionRequired(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_text_required(&self) -> Option<&ReportStoryResultTextRequired> {
    if let ReportStoryResult::TextRequired(t) = self {
      return Some(t);
    }
    None
  }

  pub fn report_story<T: AsRef<ReportStory>>(t: T) -> Self {
    ReportStoryResult::ReportStory(t.as_ref().clone())
  }

  pub fn ok<T: AsRef<ReportStoryResultOk>>(t: T) -> Self {
    ReportStoryResult::Ok(t.as_ref().clone())
  }

  pub fn option_required<T: AsRef<ReportStoryResultOptionRequired>>(t: T) -> Self {
    ReportStoryResult::OptionRequired(t.as_ref().clone())
  }

  pub fn text_required<T: AsRef<ReportStoryResultTextRequired>>(t: T) -> Self {
    ReportStoryResult::TextRequired(t.as_ref().clone())
  }
}

impl AsRef<ReportStoryResult> for ReportStoryResult {
  fn as_ref(&self) -> &ReportStoryResult {
    self
  }
}

/// The story was reported successfully
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportStoryResultOk {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportStoryResultOk {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportStoryResultOk"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportStoryResult for ReportStoryResultOk {}

impl ReportStoryResultOk {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportStoryResultOkBuilder {
    let inner = ReportStoryResultOk {
      td_name: "reportStoryResultOk".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportStoryResultOkBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportStoryResultOkBuilder {
  inner: ReportStoryResultOk,
}

impl RTDReportStoryResultOkBuilder {
  pub fn build(&self) -> ReportStoryResultOk {
    self.inner.clone()
  }
}

impl AsRef<ReportStoryResultOk> for ReportStoryResultOk {
  fn as_ref(&self) -> &ReportStoryResultOk {
    self
  }
}

impl AsRef<ReportStoryResultOk> for RTDReportStoryResultOkBuilder {
  fn as_ref(&self) -> &ReportStoryResultOk {
    &self.inner
  }
}

/// The user must choose an option to report the story and repeat request with the chosen option
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportStoryResultOptionRequired {
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

impl RObject for ReportStoryResultOptionRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportStoryResultOptionRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportStoryResult for ReportStoryResultOptionRequired {}

impl ReportStoryResultOptionRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportStoryResultOptionRequiredBuilder {
    let inner = ReportStoryResultOptionRequired {
      td_name: "reportStoryResultOptionRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReportStoryResultOptionRequiredBuilder { inner }
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn options(&self) -> &Vec<ReportOption> {
    &self.options
  }
}

#[doc(hidden)]
pub struct RTDReportStoryResultOptionRequiredBuilder {
  inner: ReportStoryResultOptionRequired,
}

impl RTDReportStoryResultOptionRequiredBuilder {
  pub fn build(&self) -> ReportStoryResultOptionRequired {
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

impl AsRef<ReportStoryResultOptionRequired> for ReportStoryResultOptionRequired {
  fn as_ref(&self) -> &ReportStoryResultOptionRequired {
    self
  }
}

impl AsRef<ReportStoryResultOptionRequired> for RTDReportStoryResultOptionRequiredBuilder {
  fn as_ref(&self) -> &ReportStoryResultOptionRequired {
    &self.inner
  }
}

/// The user must add additional text details to the report
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportStoryResultTextRequired {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Option identifier for the next reportStory request
  option_id: String,
  /// True, if the user can skip text adding
  is_optional: bool,
}

impl RObject for ReportStoryResultTextRequired {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportStoryResultTextRequired"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportStoryResult for ReportStoryResultTextRequired {}

impl ReportStoryResultTextRequired {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportStoryResultTextRequiredBuilder {
    let inner = ReportStoryResultTextRequired {
      td_name: "reportStoryResultTextRequired".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDReportStoryResultTextRequiredBuilder { inner }
  }

  pub fn option_id(&self) -> &String {
    &self.option_id
  }

  pub fn is_optional(&self) -> bool {
    self.is_optional
  }
}

#[doc(hidden)]
pub struct RTDReportStoryResultTextRequiredBuilder {
  inner: ReportStoryResultTextRequired,
}

impl RTDReportStoryResultTextRequiredBuilder {
  pub fn build(&self) -> ReportStoryResultTextRequired {
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

impl AsRef<ReportStoryResultTextRequired> for ReportStoryResultTextRequired {
  fn as_ref(&self) -> &ReportStoryResultTextRequired {
    self
  }
}

impl AsRef<ReportStoryResultTextRequired> for RTDReportStoryResultTextRequiredBuilder {
  fn as_ref(&self) -> &ReportStoryResultTextRequired {
    &self.inner
  }
}
