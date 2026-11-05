use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents result of cryptocurrency transfer from the current user's TON wallet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletTransferResult {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the transfer is gasless
  is_gasless: bool,
  /// Unique identifier of the sent message; use getTonWalletTransactionByMsgHash to poll information about the transaction
  msg_hash: String,
  /// The completed transaction; may be null if the transaction is still pending
  transaction: Option<TonWalletTransaction>,
}

impl RObject for TonWalletTransferResult {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletTransferResult"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletTransferResult {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletTransferResultBuilder {
    let inner = TonWalletTransferResult {
      td_name: "tonWalletTransferResult".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletTransferResultBuilder { inner }
  }

  pub fn is_gasless(&self) -> bool {
    self.is_gasless
  }

  pub fn msg_hash(&self) -> &String {
    &self.msg_hash
  }

  pub fn transaction(&self) -> &Option<TonWalletTransaction> {
    &self.transaction
  }
}

#[doc(hidden)]
pub struct RTDTonWalletTransferResultBuilder {
  inner: TonWalletTransferResult,
}

impl RTDTonWalletTransferResultBuilder {
  pub fn build(&self) -> TonWalletTransferResult {
    self.inner.clone()
  }

  pub fn is_gasless(&mut self, is_gasless: bool) -> &mut Self {
    self.inner.is_gasless = is_gasless;
    self
  }

  pub fn msg_hash<T: AsRef<str>>(&mut self, msg_hash: T) -> &mut Self {
    self.inner.msg_hash = msg_hash.as_ref().to_string();
    self
  }

  pub fn transaction<T: AsRef<TonWalletTransaction>>(&mut self, transaction: T) -> &mut Self {
    self.inner.transaction = Some(transaction.as_ref().clone());
    self
  }
}

impl AsRef<TonWalletTransferResult> for TonWalletTransferResult {
  fn as_ref(&self) -> &TonWalletTransferResult {
    self
  }
}

impl AsRef<TonWalletTransferResult> for RTDTonWalletTransferResultBuilder {
  fn as_ref(&self) -> &TonWalletTransferResult {
    &self.inner
  }
}
