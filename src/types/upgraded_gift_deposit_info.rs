use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains information needed to deposit an upgraded gift from the TON wallet of the current user to a Telegram account. The gift must be transferred by an internal message from the wallet to nft_address with gram_amount attached and with payload as the message body
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradedGiftDepositInfo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Address in TON blockchain to which the message must be sent
  nft_address: String,
  /// Body of the NFT transfer message
  payload: String,
  /// The amount of TON Grams that must be attached to the message to cover fees, in the smallest units of the cryptocurrency; excess amount will be returned to the wallet
  gram_amount: i64,
}

impl RObject for UpgradedGiftDepositInfo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "upgradedGiftDepositInfo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UpgradedGiftDepositInfo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUpgradedGiftDepositInfoBuilder {
    let inner = UpgradedGiftDepositInfo {
      td_name: "upgradedGiftDepositInfo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUpgradedGiftDepositInfoBuilder { inner }
  }

  pub fn nft_address(&self) -> &String {
    &self.nft_address
  }

  pub fn payload(&self) -> &String {
    &self.payload
  }

  pub fn gram_amount(&self) -> i64 {
    self.gram_amount
  }
}

#[doc(hidden)]
pub struct RTDUpgradedGiftDepositInfoBuilder {
  inner: UpgradedGiftDepositInfo,
}

impl RTDUpgradedGiftDepositInfoBuilder {
  pub fn build(&self) -> UpgradedGiftDepositInfo {
    self.inner.clone()
  }

  pub fn nft_address<T: AsRef<str>>(&mut self, nft_address: T) -> &mut Self {
    self.inner.nft_address = nft_address.as_ref().to_string();
    self
  }

  pub fn payload<T: AsRef<str>>(&mut self, payload: T) -> &mut Self {
    self.inner.payload = payload.as_ref().to_string();
    self
  }

  pub fn gram_amount(&mut self, gram_amount: i64) -> &mut Self {
    self.inner.gram_amount = gram_amount;
    self
  }
}

impl AsRef<UpgradedGiftDepositInfo> for UpgradedGiftDepositInfo {
  fn as_ref(&self) -> &UpgradedGiftDepositInfo {
    self
  }
}

impl AsRef<UpgradedGiftDepositInfo> for RTDUpgradedGiftDepositInfoBuilder {
  fn as_ref(&self) -> &UpgradedGiftDepositInfo {
    &self.inner
  }
}
