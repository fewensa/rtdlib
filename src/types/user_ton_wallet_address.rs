use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains TON wallet address of a user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UserTonWalletAddress {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the user; 0 if unknown
  user_id: i64,
  /// Address of the user's wallet in the TON blockchain
  wallet_address: String,
  /// Public key of the user
  public_key: String,
}

impl RObject for UserTonWalletAddress {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "userTonWalletAddress"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl UserTonWalletAddress {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDUserTonWalletAddressBuilder {
    let inner = UserTonWalletAddress {
      td_name: "userTonWalletAddress".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDUserTonWalletAddressBuilder { inner }
  }

  pub fn user_id(&self) -> i64 {
    self.user_id
  }

  pub fn wallet_address(&self) -> &String {
    &self.wallet_address
  }

  pub fn public_key(&self) -> &String {
    &self.public_key
  }
}

#[doc(hidden)]
pub struct RTDUserTonWalletAddressBuilder {
  inner: UserTonWalletAddress,
}

impl RTDUserTonWalletAddressBuilder {
  pub fn build(&self) -> UserTonWalletAddress {
    self.inner.clone()
  }

  pub fn user_id(&mut self, user_id: i64) -> &mut Self {
    self.inner.user_id = user_id;
    self
  }

  pub fn wallet_address<T: AsRef<str>>(&mut self, wallet_address: T) -> &mut Self {
    self.inner.wallet_address = wallet_address.as_ref().to_string();
    self
  }

  pub fn public_key<T: AsRef<str>>(&mut self, public_key: T) -> &mut Self {
    self.inner.public_key = public_key.as_ref().to_string();
    self
  }
}

impl AsRef<UserTonWalletAddress> for UserTonWalletAddress {
  fn as_ref(&self) -> &UserTonWalletAddress {
    self
  }
}

impl AsRef<UserTonWalletAddress> for RTDUserTonWalletAddressBuilder {
  fn as_ref(&self) -> &UserTonWalletAddress {
    &self.inner
  }
}
