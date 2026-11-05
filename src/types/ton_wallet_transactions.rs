use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of TON Gram transactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The total amount of owned TON Grams, in the smallest units of the cryptocurrency
  gram_amount: i64,
  /// List of transactions
  transactions: Vec<TonWalletTransaction>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for TonWalletTransactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletTransactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionsBuilder {
    let inner = TonWalletTransactions {
      td_name: "tonWalletTransactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransactionsBuilder { inner }
  }

  pub fn gram_amount(&self) -> i64 {
    self.gram_amount
  }

  pub fn transactions(&self) -> &Vec<TonWalletTransaction> {
    &self.transactions
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionsBuilder {
  inner: TonWalletTransactions,
}

impl RTDTonWalletTransactionsBuilder {
  pub fn build(&self) -> TonWalletTransactions {
    self.inner.clone()
  }

  pub fn gram_amount(&mut self, gram_amount: i64) -> &mut Self {
    self.inner.gram_amount = gram_amount;
    self
  }

  pub fn transactions(&mut self, transactions: Vec<TonWalletTransaction>) -> &mut Self {
    self.inner.transactions = transactions;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletTransactions> for TonWalletTransactions {
  fn as_ref(&self) -> &TonWalletTransactions {
    self
  }
}

impl AsRef<TonWalletTransactions> for RTDTonWalletTransactionsBuilder {
  fn as_ref(&self) -> &TonWalletTransactions {
    &self.inner
  }
}
