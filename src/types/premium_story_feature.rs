use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes a story feature available to Premium users
pub trait TDPremiumStoryFeature: Debug + RObject {}

/// Describes a story feature available to Premium users
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PremiumStoryFeature {
  #[doc(hidden)]
  _Default(()),
  /// The ability to set custom expiration duration for stories
  CustomExpirationDuration(PremiumStoryFeatureCustomExpirationDuration),
  /// The ability to use links and formatting in story caption, and use inputStoryAreaTypeLink areas
  LinksAndFormatting(PremiumStoryFeatureLinksAndFormatting),
  /// The ability to check who opened the current user's stories after they expire
  PermanentViewsHistory(PremiumStoryFeaturePermanentViewsHistory),
  /// Stories of the current user are displayed before stories of non-Premium contacts, supergroups, and channels
  PriorityOrder(PremiumStoryFeaturePriorityOrder),
  /// The ability to save others' unprotected stories
  SaveStories(PremiumStoryFeatureSaveStories),
  /// The ability to hide the fact that the user viewed others' stories
  StealthMode(PremiumStoryFeatureStealthMode),
  /// The ability to choose better quality for viewed stories
  VideoQuality(PremiumStoryFeatureVideoQuality),
}

impl Default for PremiumStoryFeature {
  fn default() -> Self {
    PremiumStoryFeature::_Default(())
  }
}

impl<'de> Deserialize<'de> for PremiumStoryFeature {
  fn deserialize<D>(deserializer: D) -> Result<PremiumStoryFeature, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      PremiumStoryFeature,
      (premiumStoryFeatureCustomExpirationDuration, CustomExpirationDuration);
      (premiumStoryFeatureLinksAndFormatting, LinksAndFormatting);
      (premiumStoryFeaturePermanentViewsHistory, PermanentViewsHistory);
      (premiumStoryFeaturePriorityOrder, PriorityOrder);
      (premiumStoryFeatureSaveStories, SaveStories);
      (premiumStoryFeatureStealthMode, StealthMode);
      (premiumStoryFeatureVideoQuality, VideoQuality);

    )(deserializer)
  }
}

impl RObject for PremiumStoryFeature {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      PremiumStoryFeature::CustomExpirationDuration(t) => t.td_name(),
      PremiumStoryFeature::LinksAndFormatting(t) => t.td_name(),
      PremiumStoryFeature::PermanentViewsHistory(t) => t.td_name(),
      PremiumStoryFeature::PriorityOrder(t) => t.td_name(),
      PremiumStoryFeature::SaveStories(t) => t.td_name(),
      PremiumStoryFeature::StealthMode(t) => t.td_name(),
      PremiumStoryFeature::VideoQuality(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      PremiumStoryFeature::CustomExpirationDuration(t) => t.extra(),
      PremiumStoryFeature::LinksAndFormatting(t) => t.extra(),
      PremiumStoryFeature::PermanentViewsHistory(t) => t.extra(),
      PremiumStoryFeature::PriorityOrder(t) => t.extra(),
      PremiumStoryFeature::SaveStories(t) => t.extra(),
      PremiumStoryFeature::StealthMode(t) => t.extra(),
      PremiumStoryFeature::VideoQuality(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl PremiumStoryFeature {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, PremiumStoryFeature::_Default(_))
  }

  pub fn is_custom_expiration_duration(&self) -> bool {
    matches!(self, PremiumStoryFeature::CustomExpirationDuration(_))
  }
  pub fn is_links_and_formatting(&self) -> bool {
    matches!(self, PremiumStoryFeature::LinksAndFormatting(_))
  }
  pub fn is_permanent_views_history(&self) -> bool {
    matches!(self, PremiumStoryFeature::PermanentViewsHistory(_))
  }
  pub fn is_priority_order(&self) -> bool {
    matches!(self, PremiumStoryFeature::PriorityOrder(_))
  }
  pub fn is_save_stories(&self) -> bool {
    matches!(self, PremiumStoryFeature::SaveStories(_))
  }
  pub fn is_stealth_mode(&self) -> bool {
    matches!(self, PremiumStoryFeature::StealthMode(_))
  }
  pub fn is_video_quality(&self) -> bool {
    matches!(self, PremiumStoryFeature::VideoQuality(_))
  }

  pub fn on_custom_expiration_duration<F: FnOnce(&PremiumStoryFeatureCustomExpirationDuration)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumStoryFeature::CustomExpirationDuration(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_links_and_formatting<F: FnOnce(&PremiumStoryFeatureLinksAndFormatting)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumStoryFeature::LinksAndFormatting(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_permanent_views_history<F: FnOnce(&PremiumStoryFeaturePermanentViewsHistory)>(
    &self,
    fnc: F,
  ) -> &Self {
    if let PremiumStoryFeature::PermanentViewsHistory(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_priority_order<F: FnOnce(&PremiumStoryFeaturePriorityOrder)>(&self, fnc: F) -> &Self {
    if let PremiumStoryFeature::PriorityOrder(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_save_stories<F: FnOnce(&PremiumStoryFeatureSaveStories)>(&self, fnc: F) -> &Self {
    if let PremiumStoryFeature::SaveStories(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_stealth_mode<F: FnOnce(&PremiumStoryFeatureStealthMode)>(&self, fnc: F) -> &Self {
    if let PremiumStoryFeature::StealthMode(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_video_quality<F: FnOnce(&PremiumStoryFeatureVideoQuality)>(&self, fnc: F) -> &Self {
    if let PremiumStoryFeature::VideoQuality(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_custom_expiration_duration(
    &self,
  ) -> Option<&PremiumStoryFeatureCustomExpirationDuration> {
    if let PremiumStoryFeature::CustomExpirationDuration(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_links_and_formatting(&self) -> Option<&PremiumStoryFeatureLinksAndFormatting> {
    if let PremiumStoryFeature::LinksAndFormatting(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_permanent_views_history(&self) -> Option<&PremiumStoryFeaturePermanentViewsHistory> {
    if let PremiumStoryFeature::PermanentViewsHistory(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_priority_order(&self) -> Option<&PremiumStoryFeaturePriorityOrder> {
    if let PremiumStoryFeature::PriorityOrder(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_save_stories(&self) -> Option<&PremiumStoryFeatureSaveStories> {
    if let PremiumStoryFeature::SaveStories(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_stealth_mode(&self) -> Option<&PremiumStoryFeatureStealthMode> {
    if let PremiumStoryFeature::StealthMode(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_video_quality(&self) -> Option<&PremiumStoryFeatureVideoQuality> {
    if let PremiumStoryFeature::VideoQuality(t) = self {
      return Some(t);
    }
    None
  }

  pub fn custom_expiration_duration<T: AsRef<PremiumStoryFeatureCustomExpirationDuration>>(
    t: T,
  ) -> Self {
    PremiumStoryFeature::CustomExpirationDuration(t.as_ref().clone())
  }

  pub fn links_and_formatting<T: AsRef<PremiumStoryFeatureLinksAndFormatting>>(t: T) -> Self {
    PremiumStoryFeature::LinksAndFormatting(t.as_ref().clone())
  }

  pub fn permanent_views_history<T: AsRef<PremiumStoryFeaturePermanentViewsHistory>>(t: T) -> Self {
    PremiumStoryFeature::PermanentViewsHistory(t.as_ref().clone())
  }

  pub fn priority_order<T: AsRef<PremiumStoryFeaturePriorityOrder>>(t: T) -> Self {
    PremiumStoryFeature::PriorityOrder(t.as_ref().clone())
  }

  pub fn save_stories<T: AsRef<PremiumStoryFeatureSaveStories>>(t: T) -> Self {
    PremiumStoryFeature::SaveStories(t.as_ref().clone())
  }

  pub fn stealth_mode<T: AsRef<PremiumStoryFeatureStealthMode>>(t: T) -> Self {
    PremiumStoryFeature::StealthMode(t.as_ref().clone())
  }

  pub fn video_quality<T: AsRef<PremiumStoryFeatureVideoQuality>>(t: T) -> Self {
    PremiumStoryFeature::VideoQuality(t.as_ref().clone())
  }
}

impl AsRef<PremiumStoryFeature> for PremiumStoryFeature {
  fn as_ref(&self) -> &PremiumStoryFeature {
    self
  }
}

/// The ability to set custom expiration duration for stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureCustomExpirationDuration {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumStoryFeatureCustomExpirationDuration {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumStoryFeatureCustomExpirationDuration"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureCustomExpirationDuration {}

impl PremiumStoryFeatureCustomExpirationDuration {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStoryFeatureCustomExpirationDurationBuilder {
    let inner = PremiumStoryFeatureCustomExpirationDuration {
      td_name: "premiumStoryFeatureCustomExpirationDuration".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumStoryFeatureCustomExpirationDurationBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumStoryFeatureCustomExpirationDurationBuilder {
  inner: PremiumStoryFeatureCustomExpirationDuration,
}

impl RTDPremiumStoryFeatureCustomExpirationDurationBuilder {
  pub fn build(&self) -> PremiumStoryFeatureCustomExpirationDuration {
    self.inner.clone()
  }
}

impl AsRef<PremiumStoryFeatureCustomExpirationDuration>
  for PremiumStoryFeatureCustomExpirationDuration
{
  fn as_ref(&self) -> &PremiumStoryFeatureCustomExpirationDuration {
    self
  }
}

impl AsRef<PremiumStoryFeatureCustomExpirationDuration>
  for RTDPremiumStoryFeatureCustomExpirationDurationBuilder
{
  fn as_ref(&self) -> &PremiumStoryFeatureCustomExpirationDuration {
    &self.inner
  }
}

/// The ability to use links and formatting in story caption, and use inputStoryAreaTypeLink areas
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureLinksAndFormatting {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumStoryFeatureLinksAndFormatting {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumStoryFeatureLinksAndFormatting"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureLinksAndFormatting {}

impl PremiumStoryFeatureLinksAndFormatting {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStoryFeatureLinksAndFormattingBuilder {
    let inner = PremiumStoryFeatureLinksAndFormatting {
      td_name: "premiumStoryFeatureLinksAndFormatting".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumStoryFeatureLinksAndFormattingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumStoryFeatureLinksAndFormattingBuilder {
  inner: PremiumStoryFeatureLinksAndFormatting,
}

impl RTDPremiumStoryFeatureLinksAndFormattingBuilder {
  pub fn build(&self) -> PremiumStoryFeatureLinksAndFormatting {
    self.inner.clone()
  }
}

impl AsRef<PremiumStoryFeatureLinksAndFormatting> for PremiumStoryFeatureLinksAndFormatting {
  fn as_ref(&self) -> &PremiumStoryFeatureLinksAndFormatting {
    self
  }
}

impl AsRef<PremiumStoryFeatureLinksAndFormatting>
  for RTDPremiumStoryFeatureLinksAndFormattingBuilder
{
  fn as_ref(&self) -> &PremiumStoryFeatureLinksAndFormatting {
    &self.inner
  }
}

/// The ability to check who opened the current user's stories after they expire
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeaturePermanentViewsHistory {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumStoryFeaturePermanentViewsHistory {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumStoryFeaturePermanentViewsHistory"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumStoryFeature for PremiumStoryFeaturePermanentViewsHistory {}

impl PremiumStoryFeaturePermanentViewsHistory {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStoryFeaturePermanentViewsHistoryBuilder {
    let inner = PremiumStoryFeaturePermanentViewsHistory {
      td_name: "premiumStoryFeaturePermanentViewsHistory".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumStoryFeaturePermanentViewsHistoryBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumStoryFeaturePermanentViewsHistoryBuilder {
  inner: PremiumStoryFeaturePermanentViewsHistory,
}

impl RTDPremiumStoryFeaturePermanentViewsHistoryBuilder {
  pub fn build(&self) -> PremiumStoryFeaturePermanentViewsHistory {
    self.inner.clone()
  }
}

impl AsRef<PremiumStoryFeaturePermanentViewsHistory> for PremiumStoryFeaturePermanentViewsHistory {
  fn as_ref(&self) -> &PremiumStoryFeaturePermanentViewsHistory {
    self
  }
}

impl AsRef<PremiumStoryFeaturePermanentViewsHistory>
  for RTDPremiumStoryFeaturePermanentViewsHistoryBuilder
{
  fn as_ref(&self) -> &PremiumStoryFeaturePermanentViewsHistory {
    &self.inner
  }
}

/// Stories of the current user are displayed before stories of non-Premium contacts, supergroups, and channels
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeaturePriorityOrder {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumStoryFeaturePriorityOrder {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumStoryFeaturePriorityOrder"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumStoryFeature for PremiumStoryFeaturePriorityOrder {}

impl PremiumStoryFeaturePriorityOrder {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStoryFeaturePriorityOrderBuilder {
    let inner = PremiumStoryFeaturePriorityOrder {
      td_name: "premiumStoryFeaturePriorityOrder".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumStoryFeaturePriorityOrderBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumStoryFeaturePriorityOrderBuilder {
  inner: PremiumStoryFeaturePriorityOrder,
}

impl RTDPremiumStoryFeaturePriorityOrderBuilder {
  pub fn build(&self) -> PremiumStoryFeaturePriorityOrder {
    self.inner.clone()
  }
}

impl AsRef<PremiumStoryFeaturePriorityOrder> for PremiumStoryFeaturePriorityOrder {
  fn as_ref(&self) -> &PremiumStoryFeaturePriorityOrder {
    self
  }
}

impl AsRef<PremiumStoryFeaturePriorityOrder> for RTDPremiumStoryFeaturePriorityOrderBuilder {
  fn as_ref(&self) -> &PremiumStoryFeaturePriorityOrder {
    &self.inner
  }
}

/// The ability to save others' unprotected stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureSaveStories {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumStoryFeatureSaveStories {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumStoryFeatureSaveStories"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureSaveStories {}

impl PremiumStoryFeatureSaveStories {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStoryFeatureSaveStoriesBuilder {
    let inner = PremiumStoryFeatureSaveStories {
      td_name: "premiumStoryFeatureSaveStories".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumStoryFeatureSaveStoriesBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumStoryFeatureSaveStoriesBuilder {
  inner: PremiumStoryFeatureSaveStories,
}

impl RTDPremiumStoryFeatureSaveStoriesBuilder {
  pub fn build(&self) -> PremiumStoryFeatureSaveStories {
    self.inner.clone()
  }
}

impl AsRef<PremiumStoryFeatureSaveStories> for PremiumStoryFeatureSaveStories {
  fn as_ref(&self) -> &PremiumStoryFeatureSaveStories {
    self
  }
}

impl AsRef<PremiumStoryFeatureSaveStories> for RTDPremiumStoryFeatureSaveStoriesBuilder {
  fn as_ref(&self) -> &PremiumStoryFeatureSaveStories {
    &self.inner
  }
}

/// The ability to hide the fact that the user viewed others' stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureStealthMode {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumStoryFeatureStealthMode {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumStoryFeatureStealthMode"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureStealthMode {}

impl PremiumStoryFeatureStealthMode {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStoryFeatureStealthModeBuilder {
    let inner = PremiumStoryFeatureStealthMode {
      td_name: "premiumStoryFeatureStealthMode".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumStoryFeatureStealthModeBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumStoryFeatureStealthModeBuilder {
  inner: PremiumStoryFeatureStealthMode,
}

impl RTDPremiumStoryFeatureStealthModeBuilder {
  pub fn build(&self) -> PremiumStoryFeatureStealthMode {
    self.inner.clone()
  }
}

impl AsRef<PremiumStoryFeatureStealthMode> for PremiumStoryFeatureStealthMode {
  fn as_ref(&self) -> &PremiumStoryFeatureStealthMode {
    self
  }
}

impl AsRef<PremiumStoryFeatureStealthMode> for RTDPremiumStoryFeatureStealthModeBuilder {
  fn as_ref(&self) -> &PremiumStoryFeatureStealthMode {
    &self.inner
  }
}

/// The ability to choose better quality for viewed stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PremiumStoryFeatureVideoQuality {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for PremiumStoryFeatureVideoQuality {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "premiumStoryFeatureVideoQuality"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDPremiumStoryFeature for PremiumStoryFeatureVideoQuality {}

impl PremiumStoryFeatureVideoQuality {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDPremiumStoryFeatureVideoQualityBuilder {
    let inner = PremiumStoryFeatureVideoQuality {
      td_name: "premiumStoryFeatureVideoQuality".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDPremiumStoryFeatureVideoQualityBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDPremiumStoryFeatureVideoQualityBuilder {
  inner: PremiumStoryFeatureVideoQuality,
}

impl RTDPremiumStoryFeatureVideoQualityBuilder {
  pub fn build(&self) -> PremiumStoryFeatureVideoQuality {
    self.inner.clone()
  }
}

impl AsRef<PremiumStoryFeatureVideoQuality> for PremiumStoryFeatureVideoQuality {
  fn as_ref(&self) -> &PremiumStoryFeatureVideoQuality {
    self
  }
}

impl AsRef<PremiumStoryFeatureVideoQuality> for RTDPremiumStoryFeatureVideoQualityBuilder {
  fn as_ref(&self) -> &PremiumStoryFeatureVideoQuality {
    &self.inner
  }
}
