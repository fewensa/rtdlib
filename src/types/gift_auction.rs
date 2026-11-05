use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an auction on which a gift can be purchased
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftAuction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the auction
  id: String,
  /// Number of gifts distributed in each round
  gifts_per_round: i64,
  /// Point in time (Unix timestamp) when the auction will start
  start_date: i64,
}

impl RObject for GiftAuction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftAuction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftAuction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftAuctionBuilder {
    let inner = GiftAuction {
      td_name: "giftAuction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftAuctionBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn gifts_per_round(&self) -> i64 {
    self.gifts_per_round
  }

  pub fn start_date(&self) -> i64 {
    self.start_date
  }
}

#[doc(hidden)]
pub struct RTDGiftAuctionBuilder {
  inner: GiftAuction,
}

impl RTDGiftAuctionBuilder {
  pub fn build(&self) -> GiftAuction {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn gifts_per_round(&mut self, gifts_per_round: i64) -> &mut Self {
    self.inner.gifts_per_round = gifts_per_round;
    self
  }

  pub fn start_date(&mut self, start_date: i64) -> &mut Self {
    self.inner.start_date = start_date;
    self
  }
}

impl AsRef<GiftAuction> for GiftAuction {
  fn as_ref(&self) -> &GiftAuction {
    self
  }
}

impl AsRef<GiftAuction> for RTDGiftAuctionBuilder {
  fn as_ref(&self) -> &GiftAuction {
    &self.inner
  }
}
