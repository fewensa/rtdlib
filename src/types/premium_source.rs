use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a source from which the Premium features screen is opened
pub trait TDPremiumSource: Debug + RObject {}

/// Describes a source from which the Premium features screen is opened
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PremiumSource {
  #[doc(hidden)]
  _Default(()),
  /// A user tried to use a Business feature
  BusinessFeature(PremiumSourceBusinessFeature),
  /// A user tried to use a Premium feature
  Feature(PremiumSourceFeature),
  /// A limit was exceeded
  LimitExceeded(PremiumSourceLimitExceeded),
  /// A user opened an internal link of the type internalLinkTypePremiumFeaturesPage
  Link(PremiumSourceLink),
  /// A user opened the Premium features screen from settings
  Settings(PremiumSourceSettings),
  /// A user tried to use a Premium story feature
  StoryFeature(PremiumSourceStoryFeature),
}

impl Default for PremiumSource {
  fn default() -> Self {
    PremiumSource::_Default(())
  }
}

impl<'de> Deserialize<'de> for PremiumSource {
  fn deserialize<D>(deserializer: D) -> Result<PremiumSource, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PremiumSource,
      (premiumSourceBusinessFeature, BusinessFeature);
      (premiumSourceFeature, Feature);
      (premiumSourceLimitExceeded, LimitExceeded);
      (premiumSourceLink, Link);
      (premiumSourceSettings, Settings);
      (premiumSourceStoryFeature, StoryFeature);

    )(deserializer)
  }
}

impl RObject for PremiumSource {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PremiumSource::BusinessFeature(t) => t.td_name(),
      PremiumSource::Feature(t) => t.td_name(),
      PremiumSource::LimitExceeded(t) => t.td_name(),
      PremiumSource::Link(t) => t.td_name(),
      PremiumSource::Settings(t) => t.td_name(),
      PremiumSource::StoryFeature(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PremiumSource::BusinessFeature(t) => t.extra(),
      PremiumSource::Feature(t) => t.extra(),
      PremiumSource::LimitExceeded(t) => t.extra(),
      PremiumSource::Link(t) => t.extra(),
      PremiumSource::Settings(t) => t.extra(),
      PremiumSource::StoryFeature(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumSource {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PremiumSource::_Default(_))
  }

  pub fn is_business_feature(&self) -> bool {
    matches!(self, PremiumSource::BusinessFeature(_))
  }
  pub fn is_feature(&self) -> bool {
    matches!(self, PremiumSource::Feature(_))
  }
  pub fn is_limit_exceeded(&self) -> bool {
    matches!(self, PremiumSource::LimitExceeded(_))
  }
  pub fn is_link(&self) -> bool {
    matches!(self, PremiumSource::Link(_))
  }
  pub fn is_settings(&self) -> bool {
    matches!(self, PremiumSource::Settings(_))
  }
  pub fn is_story_feature(&self) -> bool {
    matches!(self, PremiumSource::StoryFeature(_))
  }

  pub fn on_business_feature<F: FnOnce(&PremiumSourceBusinessFeature)>(&self, fnc: F) -> &Self {
    if let PremiumSource::BusinessFeature(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_feature<F: FnOnce(&PremiumSourceFeature)>(&self, fnc: F) -> &Self {
    if let PremiumSource::Feature(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_limit_exceeded<F: FnOnce(&PremiumSourceLimitExceeded)>(&self, fnc: F) -> &Self {
    if let PremiumSource::LimitExceeded(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_link<F: FnOnce(&PremiumSourceLink)>(&self, fnc: F) -> &Self {
    if let PremiumSource::Link(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_settings<F: FnOnce(&PremiumSourceSettings)>(&self, fnc: F) -> &Self {
    if let PremiumSource::Settings(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_story_feature<F: FnOnce(&PremiumSourceStoryFeature)>(&self, fnc: F) -> &Self {
    if let PremiumSource::StoryFeature(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_business_feature(&self) -> Option<&PremiumSourceBusinessFeature> {
    if let PremiumSource::BusinessFeature(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_feature(&self) -> Option<&PremiumSourceFeature> {
    if let PremiumSource::Feature(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_limit_exceeded(&self) -> Option<&PremiumSourceLimitExceeded> {
    if let PremiumSource::LimitExceeded(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_link(&self) -> Option<&PremiumSourceLink> {
    if let PremiumSource::Link(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_settings(&self) -> Option<&PremiumSourceSettings> {
    if let PremiumSource::Settings(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_story_feature(&self) -> Option<&PremiumSourceStoryFeature> {
    if let PremiumSource::StoryFeature(t) = self {
      return Some(t);
    }
    None
  }

  pub fn business_feature<T: AsRef<PremiumSourceBusinessFeature>>(t: T) -> Self {
    PremiumSource::BusinessFeature(t.as_ref().clone())
  }

  pub fn feature<T: AsRef<PremiumSourceFeature>>(t: T) -> Self {
    PremiumSource::Feature(t.as_ref().clone())
  }

  pub fn limit_exceeded<T: AsRef<PremiumSourceLimitExceeded>>(t: T) -> Self {
    PremiumSource::LimitExceeded(t.as_ref().clone())
  }

  pub fn link<T: AsRef<PremiumSourceLink>>(t: T) -> Self {
    PremiumSource::Link(t.as_ref().clone())
  }

  pub fn settings<T: AsRef<PremiumSourceSettings>>(t: T) -> Self {
    PremiumSource::Settings(t.as_ref().clone())
  }

  pub fn story_feature<T: AsRef<PremiumSourceStoryFeature>>(t: T) -> Self {
    PremiumSource::StoryFeature(t.as_ref().clone())
  }
}

impl AsRef<PremiumSource> for PremiumSource {
  fn as_ref(&self) -> &PremiumSource {
    self
  }
}

/// A user tried to use a Business feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceBusinessFeature {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The used feature; pass null if no specific feature was used
  feature: BusinessFeature,
}

impl RObject for PremiumSourceBusinessFeature {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumSourceBusinessFeature"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumSource for PremiumSourceBusinessFeature {}

impl PremiumSourceBusinessFeature {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumSourceBusinessFeatureBuilder {
    let inner = PremiumSourceBusinessFeature {
      td_name: "premiumSourceBusinessFeature".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumSourceBusinessFeatureBuilder { inner }
  }

  pub fn feature(&self) -> &BusinessFeature {
    &self.feature
  }
}

#[doc(hidden)]
pub struct RTDPremiumSourceBusinessFeatureBuilder {
  inner: PremiumSourceBusinessFeature,
}

impl RTDPremiumSourceBusinessFeatureBuilder {
  pub fn build(&self) -> PremiumSourceBusinessFeature {
    self.inner.clone()
  }

  pub fn feature<T: AsRef<BusinessFeature>>(&mut self, feature: T) -> &mut Self {
    self.inner.feature = feature.as_ref().clone();
    self
  }
}

impl AsRef<PremiumSourceBusinessFeature> for PremiumSourceBusinessFeature {
  fn as_ref(&self) -> &PremiumSourceBusinessFeature {
    self
  }
}

impl AsRef<PremiumSourceBusinessFeature> for RTDPremiumSourceBusinessFeatureBuilder {
  fn as_ref(&self) -> &PremiumSourceBusinessFeature {
    &self.inner
  }
}

/// A user tried to use a Premium feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceFeature {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The used feature
  feature: PremiumFeature,
}

impl RObject for PremiumSourceFeature {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumSourceFeature"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumSource for PremiumSourceFeature {}

impl PremiumSourceFeature {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumSourceFeatureBuilder {
    let inner = PremiumSourceFeature {
      td_name: "premiumSourceFeature".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumSourceFeatureBuilder { inner }
  }

  pub fn feature(&self) -> &PremiumFeature {
    &self.feature
  }
}

#[doc(hidden)]
pub struct RTDPremiumSourceFeatureBuilder {
  inner: PremiumSourceFeature,
}

impl RTDPremiumSourceFeatureBuilder {
  pub fn build(&self) -> PremiumSourceFeature {
    self.inner.clone()
  }

  pub fn feature<T: AsRef<PremiumFeature>>(&mut self, feature: T) -> &mut Self {
    self.inner.feature = feature.as_ref().clone();
    self
  }
}

impl AsRef<PremiumSourceFeature> for PremiumSourceFeature {
  fn as_ref(&self) -> &PremiumSourceFeature {
    self
  }
}

impl AsRef<PremiumSourceFeature> for RTDPremiumSourceFeatureBuilder {
  fn as_ref(&self) -> &PremiumSourceFeature {
    &self.inner
  }
}

/// A limit was exceeded
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceLimitExceeded {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the exceeded limit
  limit_type: PremiumLimitType,
}

impl RObject for PremiumSourceLimitExceeded {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumSourceLimitExceeded"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumSource for PremiumSourceLimitExceeded {}

impl PremiumSourceLimitExceeded {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumSourceLimitExceededBuilder {
    let inner = PremiumSourceLimitExceeded {
      td_name: "premiumSourceLimitExceeded".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumSourceLimitExceededBuilder { inner }
  }

  pub fn limit_type(&self) -> &PremiumLimitType {
    &self.limit_type
  }
}

#[doc(hidden)]
pub struct RTDPremiumSourceLimitExceededBuilder {
  inner: PremiumSourceLimitExceeded,
}

impl RTDPremiumSourceLimitExceededBuilder {
  pub fn build(&self) -> PremiumSourceLimitExceeded {
    self.inner.clone()
  }

  pub fn limit_type<T: AsRef<PremiumLimitType>>(&mut self, limit_type: T) -> &mut Self {
    self.inner.limit_type = limit_type.as_ref().clone();
    self
  }
}

impl AsRef<PremiumSourceLimitExceeded> for PremiumSourceLimitExceeded {
  fn as_ref(&self) -> &PremiumSourceLimitExceeded {
    self
  }
}

impl AsRef<PremiumSourceLimitExceeded> for RTDPremiumSourceLimitExceededBuilder {
  fn as_ref(&self) -> &PremiumSourceLimitExceeded {
    &self.inner
  }
}

/// A user opened an internal link of the type internalLinkTypePremiumFeaturesPage
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceLink {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The referrer from the link
  referrer: String,
}

impl RObject for PremiumSourceLink {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumSourceLink"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumSource for PremiumSourceLink {}

impl PremiumSourceLink {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumSourceLinkBuilder {
    let inner = PremiumSourceLink {
      td_name: "premiumSourceLink".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumSourceLinkBuilder { inner }
  }

  pub fn referrer(&self) -> &String {
    &self.referrer
  }
}

#[doc(hidden)]
pub struct RTDPremiumSourceLinkBuilder {
  inner: PremiumSourceLink,
}

impl RTDPremiumSourceLinkBuilder {
  pub fn build(&self) -> PremiumSourceLink {
    self.inner.clone()
  }

  pub fn referrer<T: AsRef<str>>(&mut self, referrer: T) -> &mut Self {
    self.inner.referrer = referrer.as_ref().to_string();
    self
  }
}

impl AsRef<PremiumSourceLink> for PremiumSourceLink {
  fn as_ref(&self) -> &PremiumSourceLink {
    self
  }
}

impl AsRef<PremiumSourceLink> for RTDPremiumSourceLinkBuilder {
  fn as_ref(&self) -> &PremiumSourceLink {
    &self.inner
  }
}

/// A user opened the Premium features screen from settings
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceSettings {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumSourceSettings {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumSourceSettings"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumSource for PremiumSourceSettings {}

impl PremiumSourceSettings {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumSourceSettingsBuilder {
    let inner = PremiumSourceSettings {
      td_name: "premiumSourceSettings".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumSourceSettingsBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumSourceSettingsBuilder {
  inner: PremiumSourceSettings,
}

impl RTDPremiumSourceSettingsBuilder {
  pub fn build(&self) -> PremiumSourceSettings {
    self.inner.clone()
  }
}

impl AsRef<PremiumSourceSettings> for PremiumSourceSettings {
  fn as_ref(&self) -> &PremiumSourceSettings {
    self
  }
}

impl AsRef<PremiumSourceSettings> for RTDPremiumSourceSettingsBuilder {
  fn as_ref(&self) -> &PremiumSourceSettings {
    &self.inner
  }
}

/// A user tried to use a Premium story feature
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumSourceStoryFeature {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The used feature
  feature: PremiumStoryFeature,
}

impl RObject for PremiumSourceStoryFeature {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumSourceStoryFeature"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumSource for PremiumSourceStoryFeature {}

impl PremiumSourceStoryFeature {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumSourceStoryFeatureBuilder {
    let inner = PremiumSourceStoryFeature {
      td_name: "premiumSourceStoryFeature".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDPremiumSourceStoryFeatureBuilder { inner }
  }

  pub fn feature(&self) -> &PremiumStoryFeature {
    &self.feature
  }
}

#[doc(hidden)]
pub struct RTDPremiumSourceStoryFeatureBuilder {
  inner: PremiumSourceStoryFeature,
}

impl RTDPremiumSourceStoryFeatureBuilder {
  pub fn build(&self) -> PremiumSourceStoryFeature {
    self.inner.clone()
  }

  pub fn feature<T: AsRef<PremiumStoryFeature>>(&mut self, feature: T) -> &mut Self {
    self.inner.feature = feature.as_ref().clone();
    self
  }
}

impl AsRef<PremiumSourceStoryFeature> for PremiumSourceStoryFeature {
  fn as_ref(&self) -> &PremiumSourceStoryFeature {
    self
  }
}

impl AsRef<PremiumSourceStoryFeature> for RTDPremiumSourceStoryFeatureBuilder {
  fn as_ref(&self) -> &PremiumSourceStoryFeature {
    &self.inner
  }
}
