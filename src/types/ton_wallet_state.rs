use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes state of the wallet of the current user in the TON blockchain
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletState {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Address of the wallet; may be empty if none
  address: String,
  /// Public key of the wallet; may be empty if none
  public_key: String,
  /// The total amount of owned TON Grams, in the smallest units of the cryptocurrency
  gram_amount: i64,
  /// True, if the wallet is being created
  is_being_created: bool,
  /// True, if backup for the wallet key is enabled
  is_backup_enabled: bool,
  /// True, if the user can enable backup for the wallet key
  can_enable_backup: bool,
  /// True, if the wallet key can be exported using getTonWalletSecretPhrase
  can_export_phrase: bool,
}

impl RObject for TonWalletState {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletState"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletState {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletStateBuilder {
    let inner = TonWalletState {
      td_name: "tonWalletState".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletStateBuilder { inner }
  }

  pub fn address(&self) -> &String {
    &self.address
  }

  pub fn public_key(&self) -> &String {
    &self.public_key
  }

  pub fn gram_amount(&self) -> i64 {
    self.gram_amount
  }

  pub fn is_being_created(&self) -> bool {
    self.is_being_created
  }

  pub fn is_backup_enabled(&self) -> bool {
    self.is_backup_enabled
  }

  pub fn can_enable_backup(&self) -> bool {
    self.can_enable_backup
  }

  pub fn can_export_phrase(&self) -> bool {
    self.can_export_phrase
  }
}

#[doc(hidden)]
pub struct RTDTonWalletStateBuilder {
  inner: TonWalletState,
}

impl RTDTonWalletStateBuilder {
  pub fn build(&self) -> TonWalletState {
    self.inner.clone()
  }

  pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().to_string();
    self
  }

  pub fn public_key<T: AsRef<str>>(&mut self, public_key: T) -> &mut Self {
    self.inner.public_key = public_key.as_ref().to_string();
    self
  }

  pub fn gram_amount(&mut self, gram_amount: i64) -> &mut Self {
    self.inner.gram_amount = gram_amount;
    self
  }

  pub fn is_being_created(&mut self, is_being_created: bool) -> &mut Self {
    self.inner.is_being_created = is_being_created;
    self
  }

  pub fn is_backup_enabled(&mut self, is_backup_enabled: bool) -> &mut Self {
    self.inner.is_backup_enabled = is_backup_enabled;
    self
  }

  pub fn can_enable_backup(&mut self, can_enable_backup: bool) -> &mut Self {
    self.inner.can_enable_backup = can_enable_backup;
    self
  }

  pub fn can_export_phrase(&mut self, can_export_phrase: bool) -> &mut Self {
    self.inner.can_export_phrase = can_export_phrase;
    self
  }
}

impl AsRef<TonWalletState> for TonWalletState {
  fn as_ref(&self) -> &TonWalletState {
    self
  }
}

impl AsRef<TonWalletState> for RTDTonWalletStateBuilder {
  fn as_ref(&self) -> &TonWalletState {
    &self.inner
  }
}
