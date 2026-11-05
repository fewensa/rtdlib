use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a round of an auction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AuctionRound {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// 1-based number of the round
  number: i64,
  /// Duration of the round, in seconds
  duration: i64,
  /// The number of seconds for which the round will be extended if there are changes in the top winners
  extend_time: i64,
  /// The number of top winners who trigger round extension if changed
  top_winner_count: i64,
}

impl RObject for AuctionRound {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "auctionRound"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AuctionRound {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAuctionRoundBuilder {
    let inner = AuctionRound {
      td_name: "auctionRound".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAuctionRoundBuilder { inner }
  }

  pub fn number(&self) -> i64 {
    self.number
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn extend_time(&self) -> i64 {
    self.extend_time
  }

  pub fn top_winner_count(&self) -> i64 {
    self.top_winner_count
  }
}

#[doc(hidden)]
pub struct RTDAuctionRoundBuilder {
  inner: AuctionRound,
}

impl RTDAuctionRoundBuilder {
  pub fn build(&self) -> AuctionRound {
    self.inner.clone()
  }

  pub fn number(&mut self, number: i64) -> &mut Self {
    self.inner.number = number;
    self
  }

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
    self
  }

  pub fn extend_time(&mut self, extend_time: i64) -> &mut Self {
    self.inner.extend_time = extend_time;
    self
  }

  pub fn top_winner_count(&mut self, top_winner_count: i64) -> &mut Self {
    self.inner.top_winner_count = top_winner_count;
    self
  }
}

impl AsRef<AuctionRound> for AuctionRound {
  fn as_ref(&self) -> &AuctionRound {
    self
  }
}

impl AsRef<AuctionRound> for RTDAuctionRoundBuilder {
  fn as_ref(&self) -> &AuctionRound {
    &self.inner
  }
}
