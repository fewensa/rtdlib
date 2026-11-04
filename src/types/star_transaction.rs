use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a transaction changing the amount of owned Telegram Stars
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the transaction
  id: String,
  /// The amount of added owned Telegram Stars; negative for outgoing transactions
  star_amount: StarAmount,
  /// True, if the transaction is a refund of a previous transaction
  is_refund: bool,
  /// Point in time (Unix timestamp) when the transaction was completed
  date: i64,
  /// Type of the transaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: StarTransactionType,
}

impl RObject for StarTransaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarTransaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionBuilder {
    let inner = StarTransaction {
      td_name: "starTransaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn star_amount(&self) -> &StarAmount {
    &self.star_amount
  }

  pub fn is_refund(&self) -> bool {
    self.is_refund
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn type_(&self) -> &StarTransactionType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionBuilder {
  inner: StarTransaction,
}

impl RTDStarTransactionBuilder {
  pub fn build(&self) -> StarTransaction {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn star_amount<T: AsRef<StarAmount>>(&mut self, star_amount: T) -> &mut Self {
    self.inner.star_amount = star_amount.as_ref().clone();
    self
  }

  pub fn is_refund(&mut self, is_refund: bool) -> &mut Self {
    self.inner.is_refund = is_refund;
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn type_<T: AsRef<StarTransactionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<StarTransaction> for StarTransaction {
  fn as_ref(&self) -> &StarTransaction {
    self
  }
}

impl AsRef<StarTransaction> for RTDStarTransactionBuilder {
  fn as_ref(&self) -> &StarTransaction {
    &self.inner
  }
}
