use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of Telegram Star transactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StarTransactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The amount of owned Telegram Stars
  star_amount: StarAmount,
  /// List of transactions with Telegram Stars
  transactions: Vec<StarTransaction>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for StarTransactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "starTransactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StarTransactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStarTransactionsBuilder {
    let inner = StarTransactions {
      td_name: "starTransactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStarTransactionsBuilder { inner }
  }

  pub fn star_amount(&self) -> &StarAmount {
    &self.star_amount
  }

  pub fn transactions(&self) -> &Vec<StarTransaction> {
    &self.transactions
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDStarTransactionsBuilder {
  inner: StarTransactions,
}

impl RTDStarTransactionsBuilder {
  pub fn build(&self) -> StarTransactions {
    self.inner.clone()
  }

  pub fn star_amount<T: AsRef<StarAmount>>(&mut self, star_amount: T) -> &mut Self {
    self.inner.star_amount = star_amount.as_ref().clone();
    self
  }

  pub fn transactions(&mut self, transactions: Vec<StarTransaction>) -> &mut Self {
    self.inner.transactions = transactions;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<StarTransactions> for StarTransactions {
  fn as_ref(&self) -> &StarTransactions {
    self
  }
}

impl AsRef<StarTransactions> for RTDStarTransactionsBuilder {
  fn as_ref(&self) -> &StarTransactions {
    &self.inner
  }
}
