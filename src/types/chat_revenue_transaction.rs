use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains a chat revenue transaction
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatRevenueTransaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Cryptocurrency in which revenue is calculated
  cryptocurrency: String,
  /// The withdrawn amount, in the smallest units of the cryptocurrency
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  cryptocurrency_amount: isize,
  /// Type of the transaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: ChatRevenueTransactionType,
}

impl RObject for ChatRevenueTransaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "chatRevenueTransaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl ChatRevenueTransaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDChatRevenueTransactionBuilder {
    let inner = ChatRevenueTransaction {
      td_name: "chatRevenueTransaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDChatRevenueTransactionBuilder { inner }
  }

  pub fn cryptocurrency(&self) -> &String {
    &self.cryptocurrency
  }

  pub fn cryptocurrency_amount(&self) -> isize {
    self.cryptocurrency_amount
  }

  pub fn type_(&self) -> &ChatRevenueTransactionType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDChatRevenueTransactionBuilder {
  inner: ChatRevenueTransaction,
}

impl RTDChatRevenueTransactionBuilder {
  pub fn build(&self) -> ChatRevenueTransaction {
    self.inner.clone()
  }

  pub fn cryptocurrency<T: AsRef<str>>(&mut self, cryptocurrency: T) -> &mut Self {
    self.inner.cryptocurrency = cryptocurrency.as_ref().to_string();
    self
  }

  pub fn cryptocurrency_amount(&mut self, cryptocurrency_amount: isize) -> &mut Self {
    self.inner.cryptocurrency_amount = cryptocurrency_amount;
    self
  }

  pub fn type_<T: AsRef<ChatRevenueTransactionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<ChatRevenueTransaction> for ChatRevenueTransaction {
  fn as_ref(&self) -> &ChatRevenueTransaction {
    self
  }
}

impl AsRef<ChatRevenueTransaction> for RTDChatRevenueTransactionBuilder {
  fn as_ref(&self) -> &ChatRevenueTransaction {
    &self.inner
  }
}
