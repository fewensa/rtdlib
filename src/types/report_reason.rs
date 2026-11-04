use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes the reason why a chat is reported
pub trait TDReportReason: Debug + RObject {}

/// Describes the reason why a chat is reported
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ReportReason {
  #[doc(hidden)]
  _Default(()),
  /// The chat has child abuse related content
  ChildAbuse(ReportReasonChildAbuse),
  /// The chat contains copyrighted content
  Copyright(ReportReasonCopyright),
  /// A custom reason provided by the user
  Custom(ReportReasonCustom),
  /// The chat represents a fake account
  Fake(ReportReasonFake),
  /// The chat has illegal drugs related content
  IllegalDrugs(ReportReasonIllegalDrugs),
  /// The chat contains messages with personal details
  PersonalDetails(ReportReasonPersonalDetails),
  /// The chat contains pornographic messages
  Pornography(ReportReasonPornography),
  /// The chat contains spam messages
  Spam(ReportReasonSpam),
  /// The location-based chat is unrelated to its stated location
  UnrelatedLocation(ReportReasonUnrelatedLocation),
  /// The chat promotes violence
  Violence(ReportReasonViolence),
}

impl Default for ReportReason {
  fn default() -> Self {
    ReportReason::_Default(())
  }
}

impl<'de> Deserialize<'de> for ReportReason {
  fn deserialize<D>(deserializer: D) -> Result<ReportReason, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      ReportReason,
      (reportReasonChildAbuse, ChildAbuse);
      (reportReasonCopyright, Copyright);
      (reportReasonCustom, Custom);
      (reportReasonFake, Fake);
      (reportReasonIllegalDrugs, IllegalDrugs);
      (reportReasonPersonalDetails, PersonalDetails);
      (reportReasonPornography, Pornography);
      (reportReasonSpam, Spam);
      (reportReasonUnrelatedLocation, UnrelatedLocation);
      (reportReasonViolence, Violence);

    )(deserializer)
  }
}

impl RObject for ReportReason {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      ReportReason::ChildAbuse(t) => t.td_name(),
      ReportReason::Copyright(t) => t.td_name(),
      ReportReason::Custom(t) => t.td_name(),
      ReportReason::Fake(t) => t.td_name(),
      ReportReason::IllegalDrugs(t) => t.td_name(),
      ReportReason::PersonalDetails(t) => t.td_name(),
      ReportReason::Pornography(t) => t.td_name(),
      ReportReason::Spam(t) => t.td_name(),
      ReportReason::UnrelatedLocation(t) => t.td_name(),
      ReportReason::Violence(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      ReportReason::ChildAbuse(t) => t.extra(),
      ReportReason::Copyright(t) => t.extra(),
      ReportReason::Custom(t) => t.extra(),
      ReportReason::Fake(t) => t.extra(),
      ReportReason::IllegalDrugs(t) => t.extra(),
      ReportReason::PersonalDetails(t) => t.extra(),
      ReportReason::Pornography(t) => t.extra(),
      ReportReason::Spam(t) => t.extra(),
      ReportReason::UnrelatedLocation(t) => t.extra(),
      ReportReason::Violence(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ReportReason {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, ReportReason::_Default(_))
  }

  pub fn is_child_abuse(&self) -> bool {
    matches!(self, ReportReason::ChildAbuse(_))
  }
  pub fn is_copyright(&self) -> bool {
    matches!(self, ReportReason::Copyright(_))
  }
  pub fn is_custom(&self) -> bool {
    matches!(self, ReportReason::Custom(_))
  }
  pub fn is_fake(&self) -> bool {
    matches!(self, ReportReason::Fake(_))
  }
  pub fn is_illegal_drugs(&self) -> bool {
    matches!(self, ReportReason::IllegalDrugs(_))
  }
  pub fn is_personal_details(&self) -> bool {
    matches!(self, ReportReason::PersonalDetails(_))
  }
  pub fn is_pornography(&self) -> bool {
    matches!(self, ReportReason::Pornography(_))
  }
  pub fn is_spam(&self) -> bool {
    matches!(self, ReportReason::Spam(_))
  }
  pub fn is_unrelated_location(&self) -> bool {
    matches!(self, ReportReason::UnrelatedLocation(_))
  }
  pub fn is_violence(&self) -> bool {
    matches!(self, ReportReason::Violence(_))
  }

  pub fn on_child_abuse<F: FnOnce(&ReportReasonChildAbuse)>(&self, fnc: F) -> &Self {
    if let ReportReason::ChildAbuse(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_copyright<F: FnOnce(&ReportReasonCopyright)>(&self, fnc: F) -> &Self {
    if let ReportReason::Copyright(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_custom<F: FnOnce(&ReportReasonCustom)>(&self, fnc: F) -> &Self {
    if let ReportReason::Custom(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_fake<F: FnOnce(&ReportReasonFake)>(&self, fnc: F) -> &Self {
    if let ReportReason::Fake(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_illegal_drugs<F: FnOnce(&ReportReasonIllegalDrugs)>(&self, fnc: F) -> &Self {
    if let ReportReason::IllegalDrugs(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_personal_details<F: FnOnce(&ReportReasonPersonalDetails)>(&self, fnc: F) -> &Self {
    if let ReportReason::PersonalDetails(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_pornography<F: FnOnce(&ReportReasonPornography)>(&self, fnc: F) -> &Self {
    if let ReportReason::Pornography(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_spam<F: FnOnce(&ReportReasonSpam)>(&self, fnc: F) -> &Self {
    if let ReportReason::Spam(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_unrelated_location<F: FnOnce(&ReportReasonUnrelatedLocation)>(&self, fnc: F) -> &Self {
    if let ReportReason::UnrelatedLocation(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_violence<F: FnOnce(&ReportReasonViolence)>(&self, fnc: F) -> &Self {
    if let ReportReason::Violence(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_child_abuse(&self) -> Option<&ReportReasonChildAbuse> {
    if let ReportReason::ChildAbuse(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_copyright(&self) -> Option<&ReportReasonCopyright> {
    if let ReportReason::Copyright(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_custom(&self) -> Option<&ReportReasonCustom> {
    if let ReportReason::Custom(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_fake(&self) -> Option<&ReportReasonFake> {
    if let ReportReason::Fake(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_illegal_drugs(&self) -> Option<&ReportReasonIllegalDrugs> {
    if let ReportReason::IllegalDrugs(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_personal_details(&self) -> Option<&ReportReasonPersonalDetails> {
    if let ReportReason::PersonalDetails(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_pornography(&self) -> Option<&ReportReasonPornography> {
    if let ReportReason::Pornography(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_spam(&self) -> Option<&ReportReasonSpam> {
    if let ReportReason::Spam(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_unrelated_location(&self) -> Option<&ReportReasonUnrelatedLocation> {
    if let ReportReason::UnrelatedLocation(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_violence(&self) -> Option<&ReportReasonViolence> {
    if let ReportReason::Violence(t) = self {
      return Some(t);
    }
    None
  }

  pub fn child_abuse<T: AsRef<ReportReasonChildAbuse>>(t: T) -> Self {
    ReportReason::ChildAbuse(t.as_ref().clone())
  }

  pub fn copyright<T: AsRef<ReportReasonCopyright>>(t: T) -> Self {
    ReportReason::Copyright(t.as_ref().clone())
  }

  pub fn custom<T: AsRef<ReportReasonCustom>>(t: T) -> Self {
    ReportReason::Custom(t.as_ref().clone())
  }

  pub fn fake<T: AsRef<ReportReasonFake>>(t: T) -> Self {
    ReportReason::Fake(t.as_ref().clone())
  }

  pub fn illegal_drugs<T: AsRef<ReportReasonIllegalDrugs>>(t: T) -> Self {
    ReportReason::IllegalDrugs(t.as_ref().clone())
  }

  pub fn personal_details<T: AsRef<ReportReasonPersonalDetails>>(t: T) -> Self {
    ReportReason::PersonalDetails(t.as_ref().clone())
  }

  pub fn pornography<T: AsRef<ReportReasonPornography>>(t: T) -> Self {
    ReportReason::Pornography(t.as_ref().clone())
  }

  pub fn spam<T: AsRef<ReportReasonSpam>>(t: T) -> Self {
    ReportReason::Spam(t.as_ref().clone())
  }

  pub fn unrelated_location<T: AsRef<ReportReasonUnrelatedLocation>>(t: T) -> Self {
    ReportReason::UnrelatedLocation(t.as_ref().clone())
  }

  pub fn violence<T: AsRef<ReportReasonViolence>>(t: T) -> Self {
    ReportReason::Violence(t.as_ref().clone())
  }
}

impl AsRef<ReportReason> for ReportReason {
  fn as_ref(&self) -> &ReportReason {
    self
  }
}

/// The chat has child abuse related content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonChildAbuse {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonChildAbuse {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonChildAbuse"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonChildAbuse {}

impl ReportReasonChildAbuse {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonChildAbuseBuilder {
    let inner = ReportReasonChildAbuse {
      td_name: "reportReasonChildAbuse".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonChildAbuseBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonChildAbuseBuilder {
  inner: ReportReasonChildAbuse,
}

impl RTDReportReasonChildAbuseBuilder {
  pub fn build(&self) -> ReportReasonChildAbuse {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonChildAbuse> for ReportReasonChildAbuse {
  fn as_ref(&self) -> &ReportReasonChildAbuse {
    self
  }
}

impl AsRef<ReportReasonChildAbuse> for RTDReportReasonChildAbuseBuilder {
  fn as_ref(&self) -> &ReportReasonChildAbuse {
    &self.inner
  }
}

/// The chat contains copyrighted content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonCopyright {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonCopyright {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonCopyright"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonCopyright {}

impl ReportReasonCopyright {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonCopyrightBuilder {
    let inner = ReportReasonCopyright {
      td_name: "reportReasonCopyright".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonCopyrightBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonCopyrightBuilder {
  inner: ReportReasonCopyright,
}

impl RTDReportReasonCopyrightBuilder {
  pub fn build(&self) -> ReportReasonCopyright {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonCopyright> for ReportReasonCopyright {
  fn as_ref(&self) -> &ReportReasonCopyright {
    self
  }
}

impl AsRef<ReportReasonCopyright> for RTDReportReasonCopyrightBuilder {
  fn as_ref(&self) -> &ReportReasonCopyright {
    &self.inner
  }
}

/// A custom reason provided by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonCustom {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonCustom {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonCustom"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonCustom {}

impl ReportReasonCustom {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonCustomBuilder {
    let inner = ReportReasonCustom {
      td_name: "reportReasonCustom".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonCustomBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonCustomBuilder {
  inner: ReportReasonCustom,
}

impl RTDReportReasonCustomBuilder {
  pub fn build(&self) -> ReportReasonCustom {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonCustom> for ReportReasonCustom {
  fn as_ref(&self) -> &ReportReasonCustom {
    self
  }
}

impl AsRef<ReportReasonCustom> for RTDReportReasonCustomBuilder {
  fn as_ref(&self) -> &ReportReasonCustom {
    &self.inner
  }
}

/// The chat represents a fake account
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonFake {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonFake {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonFake"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonFake {}

impl ReportReasonFake {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonFakeBuilder {
    let inner = ReportReasonFake {
      td_name: "reportReasonFake".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonFakeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonFakeBuilder {
  inner: ReportReasonFake,
}

impl RTDReportReasonFakeBuilder {
  pub fn build(&self) -> ReportReasonFake {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonFake> for ReportReasonFake {
  fn as_ref(&self) -> &ReportReasonFake {
    self
  }
}

impl AsRef<ReportReasonFake> for RTDReportReasonFakeBuilder {
  fn as_ref(&self) -> &ReportReasonFake {
    &self.inner
  }
}

/// The chat has illegal drugs related content
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonIllegalDrugs {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonIllegalDrugs {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonIllegalDrugs"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonIllegalDrugs {}

impl ReportReasonIllegalDrugs {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonIllegalDrugsBuilder {
    let inner = ReportReasonIllegalDrugs {
      td_name: "reportReasonIllegalDrugs".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonIllegalDrugsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonIllegalDrugsBuilder {
  inner: ReportReasonIllegalDrugs,
}

impl RTDReportReasonIllegalDrugsBuilder {
  pub fn build(&self) -> ReportReasonIllegalDrugs {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonIllegalDrugs> for ReportReasonIllegalDrugs {
  fn as_ref(&self) -> &ReportReasonIllegalDrugs {
    self
  }
}

impl AsRef<ReportReasonIllegalDrugs> for RTDReportReasonIllegalDrugsBuilder {
  fn as_ref(&self) -> &ReportReasonIllegalDrugs {
    &self.inner
  }
}

/// The chat contains messages with personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonPersonalDetails {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonPersonalDetails {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonPersonalDetails"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonPersonalDetails {}

impl ReportReasonPersonalDetails {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonPersonalDetailsBuilder {
    let inner = ReportReasonPersonalDetails {
      td_name: "reportReasonPersonalDetails".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonPersonalDetailsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonPersonalDetailsBuilder {
  inner: ReportReasonPersonalDetails,
}

impl RTDReportReasonPersonalDetailsBuilder {
  pub fn build(&self) -> ReportReasonPersonalDetails {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonPersonalDetails> for ReportReasonPersonalDetails {
  fn as_ref(&self) -> &ReportReasonPersonalDetails {
    self
  }
}

impl AsRef<ReportReasonPersonalDetails> for RTDReportReasonPersonalDetailsBuilder {
  fn as_ref(&self) -> &ReportReasonPersonalDetails {
    &self.inner
  }
}

/// The chat contains pornographic messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonPornography {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonPornography {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonPornography"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonPornography {}

impl ReportReasonPornography {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonPornographyBuilder {
    let inner = ReportReasonPornography {
      td_name: "reportReasonPornography".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonPornographyBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonPornographyBuilder {
  inner: ReportReasonPornography,
}

impl RTDReportReasonPornographyBuilder {
  pub fn build(&self) -> ReportReasonPornography {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonPornography> for ReportReasonPornography {
  fn as_ref(&self) -> &ReportReasonPornography {
    self
  }
}

impl AsRef<ReportReasonPornography> for RTDReportReasonPornographyBuilder {
  fn as_ref(&self) -> &ReportReasonPornography {
    &self.inner
  }
}

/// The chat contains spam messages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonSpam {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonSpam {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonSpam"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonSpam {}

impl ReportReasonSpam {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonSpamBuilder {
    let inner = ReportReasonSpam {
      td_name: "reportReasonSpam".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonSpamBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonSpamBuilder {
  inner: ReportReasonSpam,
}

impl RTDReportReasonSpamBuilder {
  pub fn build(&self) -> ReportReasonSpam {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonSpam> for ReportReasonSpam {
  fn as_ref(&self) -> &ReportReasonSpam {
    self
  }
}

impl AsRef<ReportReasonSpam> for RTDReportReasonSpamBuilder {
  fn as_ref(&self) -> &ReportReasonSpam {
    &self.inner
  }
}

/// The location-based chat is unrelated to its stated location
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonUnrelatedLocation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonUnrelatedLocation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonUnrelatedLocation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonUnrelatedLocation {}

impl ReportReasonUnrelatedLocation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonUnrelatedLocationBuilder {
    let inner = ReportReasonUnrelatedLocation {
      td_name: "reportReasonUnrelatedLocation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonUnrelatedLocationBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonUnrelatedLocationBuilder {
  inner: ReportReasonUnrelatedLocation,
}

impl RTDReportReasonUnrelatedLocationBuilder {
  pub fn build(&self) -> ReportReasonUnrelatedLocation {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonUnrelatedLocation> for ReportReasonUnrelatedLocation {
  fn as_ref(&self) -> &ReportReasonUnrelatedLocation {
    self
  }
}

impl AsRef<ReportReasonUnrelatedLocation> for RTDReportReasonUnrelatedLocationBuilder {
  fn as_ref(&self) -> &ReportReasonUnrelatedLocation {
    &self.inner
  }
}

/// The chat promotes violence
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportReasonViolence {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for ReportReasonViolence {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "reportReasonViolence"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDReportReason for ReportReasonViolence {}

impl ReportReasonViolence {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDReportReasonViolenceBuilder {
    let inner = ReportReasonViolence {
      td_name: "reportReasonViolence".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDReportReasonViolenceBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDReportReasonViolenceBuilder {
  inner: ReportReasonViolence,
}

impl RTDReportReasonViolenceBuilder {
  pub fn build(&self) -> ReportReasonViolence {
    self.inner.clone()
  }
}

impl AsRef<ReportReasonViolence> for ReportReasonViolence {
  fn as_ref(&self) -> &ReportReasonViolence {
    self
  }
}

impl AsRef<ReportReasonViolence> for RTDReportReasonViolenceBuilder {
  fn as_ref(&self) -> &ReportReasonViolence {
    &self.inner
  }
}
