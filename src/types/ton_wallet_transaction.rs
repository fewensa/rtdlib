use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a transaction with the TON wallet of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransaction {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the transaction
  id: String,
  /// Address in the TON blockchain of the transaction peer; may be empty if the transaction isn't a wallet transfer
  peer_address: String,
  /// Identifier of the other user who participated in the transaction; 0 if unknown
  peer_user_id: i64,
  /// Identifier of the domain assigned to the peer address; may be empty if none
  peer_domain: String,
  /// Point in time (Unix timestamp) when the transaction was completed
  date: i64,
  /// The fee amount paid for the transaction in TON Grams, in the smallest units of the cryptocurrency
  fee_amount: i64,
  /// State of the transaction
  state: TonWalletTransactionState,
  /// Type of the transaction
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: TonWalletTransactionType,
}

impl RObject for TonWalletTransaction {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransaction"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletTransaction {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransactionBuilder {
    let inner = TonWalletTransaction {
      td_name: "tonWalletTransaction".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransactionBuilder { inner }
  }

  pub fn id(&self) -> &String {
    &self.id
  }

  pub fn peer_address(&self) -> &String {
    &self.peer_address
  }

  pub fn peer_user_id(&self) -> i64 {
    self.peer_user_id
  }

  pub fn peer_domain(&self) -> &String {
    &self.peer_domain
  }

  pub fn date(&self) -> i64 {
    self.date
  }

  pub fn fee_amount(&self) -> i64 {
    self.fee_amount
  }

  pub fn state(&self) -> &TonWalletTransactionState {
    &self.state
  }

  pub fn type_(&self) -> &TonWalletTransactionType {
    &self.type_
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransactionBuilder {
  inner: TonWalletTransaction,
}

impl RTDTonWalletTransactionBuilder {
  pub fn build(&self) -> TonWalletTransaction {
    self.inner.clone()
  }

  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

  pub fn peer_address<T: AsRef<str>>(&mut self, peer_address: T) -> &mut Self {
    self.inner.peer_address = peer_address.as_ref().to_string();
    self
  }

  pub fn peer_user_id(&mut self, peer_user_id: i64) -> &mut Self {
    self.inner.peer_user_id = peer_user_id;
    self
  }

  pub fn peer_domain<T: AsRef<str>>(&mut self, peer_domain: T) -> &mut Self {
    self.inner.peer_domain = peer_domain.as_ref().to_string();
    self
  }

  pub fn date(&mut self, date: i64) -> &mut Self {
    self.inner.date = date;
    self
  }

  pub fn fee_amount(&mut self, fee_amount: i64) -> &mut Self {
    self.inner.fee_amount = fee_amount;
    self
  }

  pub fn state<T: AsRef<TonWalletTransactionState>>(&mut self, state: T) -> &mut Self {
    self.inner.state = state.as_ref().clone();
    self
  }

  pub fn type_<T: AsRef<TonWalletTransactionType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }
}

impl AsRef<TonWalletTransaction> for TonWalletTransaction {
  fn as_ref(&self) -> &TonWalletTransaction {
    self
  }
}

impl AsRef<TonWalletTransaction> for RTDTonWalletTransactionBuilder {
  fn as_ref(&self) -> &TonWalletTransaction {
    &self.inner
  }
}
