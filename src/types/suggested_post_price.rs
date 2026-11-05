use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes price of a suggested post
pub trait TDSuggestedPostPrice: Debug + RObject {}

/// Describes price of a suggested post
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum SuggestedPostPrice {
  #[doc(hidden)]
  _Default(()),
  /// Describes price of a suggested post in TON Grams
  Gram(SuggestedPostPriceGram),
  /// Describes price of a suggested post in Telegram Stars
  Star(SuggestedPostPriceStar),
}

impl Default for SuggestedPostPrice {
  fn default() -> Self {
    SuggestedPostPrice::_Default(())
  }
}

impl<'de> Deserialize<'de> for SuggestedPostPrice {
  fn deserialize<D>(deserializer: D) -> Result<SuggestedPostPrice, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      SuggestedPostPrice,
      (suggestedPostPriceGram, Gram);
      (suggestedPostPriceStar, Star);

    )(deserializer)
  }
}

impl RObject for SuggestedPostPrice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      SuggestedPostPrice::Gram(t) => t.td_name(),
      SuggestedPostPrice::Star(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      SuggestedPostPrice::Gram(t) => t.extra(),
      SuggestedPostPrice::Star(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl SuggestedPostPrice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, SuggestedPostPrice::_Default(_))
  }

  pub fn is_gram(&self) -> bool {
    matches!(self, SuggestedPostPrice::Gram(_))
  }
  pub fn is_star(&self) -> bool {
    matches!(self, SuggestedPostPrice::Star(_))
  }

  pub fn on_gram<F: FnOnce(&SuggestedPostPriceGram)>(&self, fnc: F) -> &Self {
    if let SuggestedPostPrice::Gram(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_star<F: FnOnce(&SuggestedPostPriceStar)>(&self, fnc: F) -> &Self {
    if let SuggestedPostPrice::Star(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_gram(&self) -> Option<&SuggestedPostPriceGram> {
    if let SuggestedPostPrice::Gram(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_star(&self) -> Option<&SuggestedPostPriceStar> {
    if let SuggestedPostPrice::Star(t) = self {
      return Some(t);
    }
    None
  }

  pub fn gram<T: AsRef<SuggestedPostPriceGram>>(t: T) -> Self {
    SuggestedPostPrice::Gram(t.as_ref().clone())
  }

  pub fn star<T: AsRef<SuggestedPostPriceStar>>(t: T) -> Self {
    SuggestedPostPrice::Star(t.as_ref().clone())
  }
}

impl AsRef<SuggestedPostPrice> for SuggestedPostPrice {
  fn as_ref(&self) -> &SuggestedPostPrice {
    self
  }
}

/// Describes price of a suggested post in TON Grams
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedPostPriceGram {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The amount of 1/100 of Gram expected to be paid for the post; getOption("suggested_post_gram_cent_count_min")-getOption("suggested_post_gram_cent_count_max")
  gram_cent_count: i64,
}

impl RObject for SuggestedPostPriceGram {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedPostPriceGram"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedPostPrice for SuggestedPostPriceGram {}

impl SuggestedPostPriceGram {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedPostPriceGramBuilder {
    let inner = SuggestedPostPriceGram {
      td_name: "suggestedPostPriceGram".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSuggestedPostPriceGramBuilder { inner }
  }

  pub fn gram_cent_count(&self) -> i64 {
    self.gram_cent_count
  }
}

#[doc(hidden)]
pub struct RTDSuggestedPostPriceGramBuilder {
  inner: SuggestedPostPriceGram,
}

impl RTDSuggestedPostPriceGramBuilder {
  pub fn build(&self) -> SuggestedPostPriceGram {
    self.inner.clone()
  }

  pub fn gram_cent_count(&mut self, gram_cent_count: i64) -> &mut Self {
    self.inner.gram_cent_count = gram_cent_count;
    self
  }
}

impl AsRef<SuggestedPostPriceGram> for SuggestedPostPriceGram {
  fn as_ref(&self) -> &SuggestedPostPriceGram {
    self
  }
}

impl AsRef<SuggestedPostPriceGram> for RTDSuggestedPostPriceGramBuilder {
  fn as_ref(&self) -> &SuggestedPostPriceGram {
    &self.inner
  }
}

/// Describes price of a suggested post in Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SuggestedPostPriceStar {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The Telegram Star amount expected to be paid for the post; getOption("suggested_post_star_count_min")-getOption("suggested_post_star_count_max")
  star_count: i64,
}

impl RObject for SuggestedPostPriceStar {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "suggestedPostPriceStar"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDSuggestedPostPrice for SuggestedPostPriceStar {}

impl SuggestedPostPriceStar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDSuggestedPostPriceStarBuilder {
    let inner = SuggestedPostPriceStar {
      td_name: "suggestedPostPriceStar".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDSuggestedPostPriceStarBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDSuggestedPostPriceStarBuilder {
  inner: SuggestedPostPriceStar,
}

impl RTDSuggestedPostPriceStarBuilder {
  pub fn build(&self) -> SuggestedPostPriceStar {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<SuggestedPostPriceStar> for SuggestedPostPriceStar {
  fn as_ref(&self) -> &SuggestedPostPriceStar {
    self
  }
}

impl AsRef<SuggestedPostPriceStar> for RTDSuggestedPostPriceStarBuilder {
  fn as_ref(&self) -> &SuggestedPostPriceStar {
    &self.inner
  }
}
