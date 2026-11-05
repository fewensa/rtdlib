use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes parameters of a unique gift available for resale
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftResaleParameters {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Resale price of the gift in Telegram Stars
  star_count: i64,
  /// Resale price of the gift in 1/100 of TON Gram
  gram_cent_count: i64,
  /// True, if the gift can be bought only using Grams
  gram_only: bool,
}

impl RObject for GiftResaleParameters {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftResaleParameters"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftResaleParameters {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftResaleParametersBuilder {
    let inner = GiftResaleParameters {
      td_name: "giftResaleParameters".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftResaleParametersBuilder { inner }
  }

  pub fn star_count(&self) -> i64 {
    self.star_count
  }

  pub fn gram_cent_count(&self) -> i64 {
    self.gram_cent_count
  }

  pub fn gram_only(&self) -> bool {
    self.gram_only
  }
}

#[doc(hidden)]
pub struct RTDGiftResaleParametersBuilder {
  inner: GiftResaleParameters,
}

impl RTDGiftResaleParametersBuilder {
  pub fn build(&self) -> GiftResaleParameters {
    self.inner.clone()
  }

  pub fn star_count(&mut self, star_count: i64) -> &mut Self {
    self.inner.star_count = star_count;
    self
  }

  pub fn gram_cent_count(&mut self, gram_cent_count: i64) -> &mut Self {
    self.inner.gram_cent_count = gram_cent_count;
    self
  }

  pub fn gram_only(&mut self, gram_only: bool) -> &mut Self {
    self.inner.gram_only = gram_only;
    self
  }
}

impl AsRef<GiftResaleParameters> for GiftResaleParameters {
  fn as_ref(&self) -> &GiftResaleParameters {
    self
  }
}

impl AsRef<GiftResaleParameters> for RTDGiftResaleParametersBuilder {
  fn as_ref(&self) -> &GiftResaleParameters {
    &self.inner
  }
}
