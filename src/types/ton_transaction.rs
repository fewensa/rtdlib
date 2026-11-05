use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a transaction changing the amount of owned TON Grams
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonTransaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the transaction
  id: String,
  /// The amount of added owned Grams, in the smallest units of the cryptocurrency; negative for outgoing transactions
  gram_amount: i64,
  /// True, if the transaction is a refund of a previous transaction
  is_refund: bool,
  /// Point in time (Unix timestamp) when the transaction was completed
  date: i64,
  /// Type of the transaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: TonTransactionType,
}

impl RObject for TonTransaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonTransaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonTransaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonTransactionBuilder {
    let inner = TonTransaction {
      td_name: "tonTransaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonTransactionBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn gram_amount(&self) -> i64 {
    self.gram_amount
  }

  pub fn is_refund(&self) -> bool {
    self.is_refund
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn type_(&self) -> &TonTransactionType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDTonTransactionBuilder {
  inner: TonTransaction,
}

impl RTDTonTransactionBuilder {
  pub fn build(&self) -> TonTransaction {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn gram_amount(&mut self, gram_amount: i64) -> &mut Self {
    self.inner.gram_amount = gram_amount;
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

  pub fn type_<T: AsRef<TonTransactionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<TonTransaction> for TonTransaction {
  fn as_ref(&self) -> &TonTransaction {
    self
  }
}

impl AsRef<TonTransaction> for RTDTonTransactionBuilder {
  fn as_ref(&self) -> &TonTransaction {
    &self.inner
  }
}
