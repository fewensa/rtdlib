use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information about an affiliate that received commission from a Telegram Star transaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AffiliateInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The number of Telegram Stars received by the affiliate for each 1000 Telegram Stars received by the program owner
  commission_per_mille: i64,
  /// Identifier of the chat which received the commission
  affiliate_chat_id: i64,
  /// The Telegram Star amount that was received by the affiliate; can be negative for refunds
  star_amount: StarAmount,
}

impl RObject for AffiliateInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "affiliateInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AffiliateInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAffiliateInfoBuilder {
    let inner = AffiliateInfo {
      td_name: "affiliateInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAffiliateInfoBuilder { inner }
  }

  pub fn commission_per_mille(&self) -> i64 {
    self.commission_per_mille
  }

  pub fn affiliate_chat_id(&self) -> i64 {
    self.affiliate_chat_id
  }

  pub fn star_amount(&self) -> &StarAmount {
    &self.star_amount
  }
}

#[doc(hidden)]
pub struct RTDAffiliateInfoBuilder {
  inner: AffiliateInfo,
}

impl RTDAffiliateInfoBuilder {
  pub fn build(&self) -> AffiliateInfo {
    self.inner.clone()
  }

  pub fn commission_per_mille(&mut self, commission_per_mille: i64) -> &mut Self {
    self.inner.commission_per_mille = commission_per_mille;
    self
  }

  pub fn affiliate_chat_id(&mut self, affiliate_chat_id: i64) -> &mut Self {
    self.inner.affiliate_chat_id = affiliate_chat_id;
    self
  }

  pub fn star_amount<T: AsRef<StarAmount>>(&mut self, star_amount: T) -> &mut Self {
    self.inner.star_amount = star_amount.as_ref().clone();
    self
  }
}

impl AsRef<AffiliateInfo> for AffiliateInfo {
  fn as_ref(&self) -> &AffiliateInfo {
    self
  }
}

impl AsRef<AffiliateInfo> for RTDAffiliateInfoBuilder {
  fn as_ref(&self) -> &AffiliateInfo {
    &self.inner
  }
}
