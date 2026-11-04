use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents state of gasless transfers for the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletGaslessTransfersInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Number of left gasless transfers. The daily available number of gasless transfers is available as getOption("ton_wallet_gasless_transfer_daily_count_max")
  left_count: i64,
  /// Point in time (Unix timestamp) when the number of left transfers will be reset; 0 if never
  reset_date: i64,
  /// Address of the relayer for gasless transfers; may be empty if unavailable
  relayer_address: String,
}

impl RObject for TonWalletGaslessTransfersInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletGaslessTransfersInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletGaslessTransfersInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletGaslessTransfersInfoBuilder {
    let inner = TonWalletGaslessTransfersInfo {
      td_name: "tonWalletGaslessTransfersInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletGaslessTransfersInfoBuilder { inner }
  }

  pub fn left_count(&self) -> i64 {
    self.left_count
  }

  pub fn reset_date(&self) -> i64 {
    self.reset_date
  }

  pub fn relayer_address(&self) -> &String {
    &self.relayer_address
  }
}

#[doc(hidden)]
pub struct RTDTonWalletGaslessTransfersInfoBuilder {
  inner: TonWalletGaslessTransfersInfo,
}

impl RTDTonWalletGaslessTransfersInfoBuilder {
  pub fn build(&self) -> TonWalletGaslessTransfersInfo {
    self.inner.clone()
  }

  pub fn left_count(&mut self, left_count: i64) -> &mut Self {
    self.inner.left_count = left_count;
    self
  }

  pub fn reset_date(&mut self, reset_date: i64) -> &mut Self {
    self.inner.reset_date = reset_date;
    self
  }

  pub fn relayer_address<T: AsRef<str>>(&mut self, relayer_address: T) -> &mut Self {
    self.inner.relayer_address = relayer_address.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletGaslessTransfersInfo> for TonWalletGaslessTransfersInfo {
  fn as_ref(&self) -> &TonWalletGaslessTransfersInfo {
    self
  }
}

impl AsRef<TonWalletGaslessTransfersInfo> for RTDTonWalletGaslessTransfersInfoBuilder {
  fn as_ref(&self) -> &TonWalletGaslessTransfersInfo {
    &self.inner
  }
}
