use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes balance of the current user in the bot https://t.me/walt
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WalletBotBalance {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// True, if the user has positive TON Gram balance
  has_balance: bool,
  /// The URL to open to launch the bot; may be empty if none
  url: String,
}

impl RObject for WalletBotBalance {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "walletBotBalance"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl WalletBotBalance {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDWalletBotBalanceBuilder {
    let inner = WalletBotBalance {
      td_name: "walletBotBalance".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDWalletBotBalanceBuilder { inner }
  }

  pub fn has_balance(&self) -> bool {
    self.has_balance
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDWalletBotBalanceBuilder {
  inner: WalletBotBalance,
}

impl RTDWalletBotBalanceBuilder {
  pub fn build(&self) -> WalletBotBalance {
    self.inner.clone()
  }

  pub fn has_balance(&mut self, has_balance: bool) -> &mut Self {
    self.inner.has_balance = has_balance;
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<WalletBotBalance> for WalletBotBalance {
  fn as_ref(&self) -> &WalletBotBalance {
    self
  }
}

impl AsRef<WalletBotBalance> for RTDWalletBotBalanceBuilder {
  fn as_ref(&self) -> &WalletBotBalance {
    &self.inner
  }
}
