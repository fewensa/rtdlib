use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of TON wallet addresses of users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTonWalletAddresses {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A list of TON wallet addresses
  addresses: Vec<UserTonWalletAddress>,
}

impl RObject for UserTonWalletAddresses {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userTonWalletAddresses"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserTonWalletAddresses {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserTonWalletAddressesBuilder {
    let inner = UserTonWalletAddresses {
      td_name: "userTonWalletAddresses".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserTonWalletAddressesBuilder { inner }
  }

  pub fn addresses(&self) -> &Vec<UserTonWalletAddress> {
    &self.addresses
  }
}

#[doc(hidden)]
pub struct RTDUserTonWalletAddressesBuilder {
  inner: UserTonWalletAddresses,
}

impl RTDUserTonWalletAddressesBuilder {
  pub fn build(&self) -> UserTonWalletAddresses {
    self.inner.clone()
  }

  pub fn addresses(&mut self, addresses: Vec<UserTonWalletAddress>) -> &mut Self {
    self.inner.addresses = addresses;
    self
  }
}

impl AsRef<UserTonWalletAddresses> for UserTonWalletAddresses {
  fn as_ref(&self) -> &UserTonWalletAddresses {
    self
  }
}

impl AsRef<UserTonWalletAddresses> for RTDUserTonWalletAddressesBuilder {
  fn as_ref(&self) -> &UserTonWalletAddresses {
    &self.inner
  }
}
