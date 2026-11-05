use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of chat revenue transactions
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueTransactions {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The amount of owned TON Grams; in the smallest units of the cryptocurrency
  gram_amount: i64,
  /// List of transactions
  transactions: Vec<ChatRevenueTransaction>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for ChatRevenueTransactions {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueTransactions"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatRevenueTransactions {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueTransactionsBuilder {
    let inner = ChatRevenueTransactions {
      td_name: "chatRevenueTransactions".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatRevenueTransactionsBuilder { inner }
  }

  pub fn gram_amount(&self) -> i64 {
    self.gram_amount
  }

  pub fn transactions(&self) -> &Vec<ChatRevenueTransaction> {
    &self.transactions
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueTransactionsBuilder {
  inner: ChatRevenueTransactions,
}

impl RTDChatRevenueTransactionsBuilder {
  pub fn build(&self) -> ChatRevenueTransactions {
    self.inner.clone()
  }

  pub fn gram_amount(&mut self, gram_amount: i64) -> &mut Self {
    self.inner.gram_amount = gram_amount;
    self
  }

  pub fn transactions(&mut self, transactions: Vec<ChatRevenueTransaction>) -> &mut Self {
    self.inner.transactions = transactions;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<ChatRevenueTransactions> for ChatRevenueTransactions {
  fn as_ref(&self) -> &ChatRevenueTransactions {
    self
  }
}

impl AsRef<ChatRevenueTransactions> for RTDChatRevenueTransactionsBuilder {
  fn as_ref(&self) -> &ChatRevenueTransactions {
    &self.inner
  }
}
