use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

use serde::de::{Deserialize, Deserializer};
use std::fmt::Debug;

/// TRAIT | Describes direction of transactions in a transaction list
pub trait TDTransactionDirection: Debug + RObject {}

/// Describes direction of transactions in a transaction list
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum TransactionDirection {
  #[doc(hidden)]
  _Default(()),
  /// The transaction is incoming and increases the amount of owned currency
  Incoming(TransactionDirectionIncoming),
  /// The transaction is outgoing and decreases the amount of owned currency
  Outgoing(TransactionDirectionOutgoing),
}

impl Default for TransactionDirection {
  fn default() -> Self {
    TransactionDirection::_Default(())
  }
}

impl<'de> Deserialize<'de> for TransactionDirection {
  fn deserialize<D>(deserializer: D) -> Result<TransactionDirection, D::Error>
  where
    D: Deserializer<'de>,
  {
    use serde::de::Error;
    rtd_enum_deserialize!(
      TransactionDirection,
      (transactionDirectionIncoming, Incoming);
      (transactionDirectionOutgoing, Outgoing);

    )(deserializer)
  }
}

impl RObject for TransactionDirection {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    match self {
      TransactionDirection::Incoming(t) => t.td_name(),
      TransactionDirection::Outgoing(t) => t.td_name(),

      _ => "-1",
    }
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    match self {
      TransactionDirection::Incoming(t) => t.extra(),
      TransactionDirection::Outgoing(t) => t.extra(),

      _ => None,
    }
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TransactionDirection {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  #[doc(hidden)]
  pub fn _is_default(&self) -> bool {
    matches!(self, TransactionDirection::_Default(_))
  }

  pub fn is_incoming(&self) -> bool {
    matches!(self, TransactionDirection::Incoming(_))
  }
  pub fn is_outgoing(&self) -> bool {
    matches!(self, TransactionDirection::Outgoing(_))
  }

  pub fn on_incoming<F: FnOnce(&TransactionDirectionIncoming)>(&self, fnc: F) -> &Self {
    if let TransactionDirection::Incoming(t) = self {
      fnc(t)
    };
    self
  }
  pub fn on_outgoing<F: FnOnce(&TransactionDirectionOutgoing)>(&self, fnc: F) -> &Self {
    if let TransactionDirection::Outgoing(t) = self {
      fnc(t)
    };
    self
  }

  pub fn as_incoming(&self) -> Option<&TransactionDirectionIncoming> {
    if let TransactionDirection::Incoming(t) = self {
      return Some(t);
    }
    None
  }
  pub fn as_outgoing(&self) -> Option<&TransactionDirectionOutgoing> {
    if let TransactionDirection::Outgoing(t) = self {
      return Some(t);
    }
    None
  }

  pub fn incoming<T: AsRef<TransactionDirectionIncoming>>(t: T) -> Self {
    TransactionDirection::Incoming(t.as_ref().clone())
  }

  pub fn outgoing<T: AsRef<TransactionDirectionOutgoing>>(t: T) -> Self {
    TransactionDirection::Outgoing(t.as_ref().clone())
  }
}

impl AsRef<TransactionDirection> for TransactionDirection {
  fn as_ref(&self) -> &TransactionDirection {
    self
  }
}

/// The transaction is incoming and increases the amount of owned currency
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionDirectionIncoming {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TransactionDirectionIncoming {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "transactionDirectionIncoming"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTransactionDirection for TransactionDirectionIncoming {}

impl TransactionDirectionIncoming {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTransactionDirectionIncomingBuilder {
    let inner = TransactionDirectionIncoming {
      td_name: "transactionDirectionIncoming".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTransactionDirectionIncomingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTransactionDirectionIncomingBuilder {
  inner: TransactionDirectionIncoming,
}

impl RTDTransactionDirectionIncomingBuilder {
  pub fn build(&self) -> TransactionDirectionIncoming {
    self.inner.clone()
  }
}

impl AsRef<TransactionDirectionIncoming> for TransactionDirectionIncoming {
  fn as_ref(&self) -> &TransactionDirectionIncoming {
    self
  }
}

impl AsRef<TransactionDirectionIncoming> for RTDTransactionDirectionIncomingBuilder {
  fn as_ref(&self) -> &TransactionDirectionIncoming {
    &self.inner
  }
}

/// The transaction is outgoing and decreases the amount of owned currency
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionDirectionOutgoing {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
}

impl RObject for TransactionDirectionOutgoing {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "transactionDirectionOutgoing"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TDTransactionDirection for TransactionDirectionOutgoing {}

impl TransactionDirectionOutgoing {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTransactionDirectionOutgoingBuilder {
    let inner = TransactionDirectionOutgoing {
      td_name: "transactionDirectionOutgoing".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
    };
    RTDTransactionDirectionOutgoingBuilder { inner }
  }
}

#[doc(hidden)]
pub struct RTDTransactionDirectionOutgoingBuilder {
  inner: TransactionDirectionOutgoing,
}

impl RTDTransactionDirectionOutgoingBuilder {
  pub fn build(&self) -> TransactionDirectionOutgoing {
    self.inner.clone()
  }
}

impl AsRef<TransactionDirectionOutgoing> for TransactionDirectionOutgoing {
  fn as_ref(&self) -> &TransactionDirectionOutgoing {
    self
  }
}

impl AsRef<TransactionDirectionOutgoing> for RTDTransactionDirectionOutgoingBuilder {
  fn as_ref(&self) -> &TransactionDirectionOutgoing {
    &self.inner
  }
}
