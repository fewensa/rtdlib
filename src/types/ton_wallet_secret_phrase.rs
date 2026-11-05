use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains the secret phrase of a TON wallet
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletSecretPhrase {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The secret phrase as a space separated list of words
  secret_phrase: String,
}

impl RObject for TonWalletSecretPhrase {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletSecretPhrase"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletSecretPhrase {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletSecretPhraseBuilder {
    let inner = TonWalletSecretPhrase {
      td_name: "tonWalletSecretPhrase".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletSecretPhraseBuilder { inner }
  }

  pub fn secret_phrase(&self) -> &String {
    &self.secret_phrase
  }
}

#[doc(hidden)]
pub struct RTDTonWalletSecretPhraseBuilder {
  inner: TonWalletSecretPhrase,
}

impl RTDTonWalletSecretPhraseBuilder {
  pub fn build(&self) -> TonWalletSecretPhrase {
    self.inner.clone()
  }

  pub fn secret_phrase<T: AsRef<str>>(&mut self, secret_phrase: T) -> &mut Self {
    self.inner.secret_phrase = secret_phrase.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletSecretPhrase> for TonWalletSecretPhrase {
  fn as_ref(&self) -> &TonWalletSecretPhrase {
    self
  }
}

impl AsRef<TonWalletSecretPhrase> for RTDTonWalletSecretPhraseBuilder {
  fn as_ref(&self) -> &TonWalletSecretPhrase {
    &self.inner
  }
}
