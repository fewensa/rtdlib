use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes price of a resold gift
pub trait TDGiftResalePrice: Debug + RObject {}

/// Describes price of a resold gift
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum GiftResalePrice {
  #[doc(hidden)]
  _Default(()),
  /// Describes price of a resold gift in TON Grams
  Gram(GiftResalePriceGram),
  /// Describes price of a resold gift in Telegram Stars
  Star(GiftResalePriceStar),
}

impl Default for GiftResalePrice {
  fn default() -> Self {
    GiftResalePrice::_Default(())
  }
}

impl<'de> Deserialize<'de> for GiftResalePrice {
  fn deserialize<D>(deserializer: D) -> Result<GiftResalePrice, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      GiftResalePrice,
      (giftResalePriceGram, Gram);
      (giftResalePriceStar, Star);

    )(deserializer)
  }
}

impl RObject for GiftResalePrice {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      GiftResalePrice::Gram(t) => t.td_name(),
      GiftResalePrice::Star(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      GiftResalePrice::Gram(t) => t.extra(),
      GiftResalePrice::Star(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftResalePrice {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, GiftResalePrice::_Default(_))
  }

  pub fn is_gram(&self) -> bool {
    matches!(self, GiftResalePrice::Gram(_))
  }
  pub fn is_star(&self) -> bool {
    matches!(self, GiftResalePrice::Star(_))
  }

  pub fn on_gram<F: FnOnce(&GiftResalePriceGram)>(&self, fnc: F) -> &Self {
    if let GiftResalePrice::Gram(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_star<F: FnOnce(&GiftResalePriceStar)>(&self, fnc: F) -> &Self {
    if let GiftResalePrice::Star(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_gram(&self) -> Option<&GiftResalePriceGram> {
    if let GiftResalePrice::Gram(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_star(&self) -> Option<&GiftResalePriceStar> {
    if let GiftResalePrice::Star(t) = self {
      return Some(t);
    }
    None
  }

  pub fn gram<T: AsRef<GiftResalePriceGram>>(t: T) -> Self {
    GiftResalePrice::Gram(t.as_ref().clone())
  }

  pub fn star<T: AsRef<GiftResalePriceStar>>(t: T) -> Self {
    GiftResalePrice::Star(t.as_ref().clone())
  }
}

impl AsRef<GiftResalePrice> for GiftResalePrice {
  fn as_ref(&self) -> &GiftResalePrice {
    self
  }
}

/// Describes price of a resold gift in TON Grams
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftResalePriceGram {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The amount of 1/100 of Gram expected to be paid for the gift. Must be in the range getOption("gift_resale_gram_cent_count_min")-getOption("gift_resale_gram_cent_count_max")
  gram_cent_count: i64,
}

impl RObject for GiftResalePriceGram {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftResalePriceGram"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftResalePrice for GiftResalePriceGram {}

impl GiftResalePriceGram {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftResalePriceGramBuilder {
    let inner = GiftResalePriceGram {
      td_name: "giftResalePriceGram".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftResalePriceGramBuilder { inner }
  }

  pub fn gram_cent_count(&self) -> i64 {
    self.gram_cent_count
  }
}

#[doc(hidden)]
pub struct RTDGiftResalePriceGramBuilder {
  inner: GiftResalePriceGram,
}

impl RTDGiftResalePriceGramBuilder {
  pub fn build(&self) -> GiftResalePriceGram {
    self.inner.clone()
  }

  pub fn gram_cent_count(&mut self, gram_cent_count: i64) -> &mut Self {
    self.inner.gram_cent_count = gram_cent_count;
    self
  }
}

impl AsRef<GiftResalePriceGram> for GiftResalePriceGram {
  fn as_ref(&self) -> &GiftResalePriceGram {
    self
  }
}

impl AsRef<GiftResalePriceGram> for RTDGiftResalePriceGramBuilder {
  fn as_ref(&self) -> &GiftResalePriceGram {
    &self.inner
  }
}

/// Describes price of a resold gift in Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftResalePriceStar {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The Telegram Star amount expected to be paid for the gift. Must be in the range getOption("gift_resale_star_count_min")-getOption("gift_resale_star_count_max") for gifts put for resale
  star_count: i64,
}

impl RObject for GiftResalePriceStar {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftResalePriceStar"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDGiftResalePrice for GiftResalePriceStar {}

impl GiftResalePriceStar {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftResalePriceStarBuilder {
    let inner = GiftResalePriceStar {
      td_name: "giftResalePriceStar".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftResalePriceStarBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }
}

#[doc(hidden)]
pub struct RTDGiftResalePriceStarBuilder {
  inner: GiftResalePriceStar,
}

impl RTDGiftResalePriceStarBuilder {
  pub fn build(&self) -> GiftResalePriceStar {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }
}

impl AsRef<GiftResalePriceStar> for GiftResalePriceStar {
  fn as_ref(&self) -> &GiftResalePriceStar {
    self
  }
}

impl AsRef<GiftResalePriceStar> for RTDGiftResalePriceStarBuilder {
  fn as_ref(&self) -> &GiftResalePriceStar {
    &self.inner
  }
}
