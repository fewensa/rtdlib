use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a TON wallet card, which is shown on the profile page of a user to everyone
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletCard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Address of the wallet
  address: String,
  /// Balance of the wallet, in the smallest units of the cryptocurrency; may be slightly outdated
  gram_amount: i64,
  /// Stickers placed on the card
  stickers: Vec<TonWalletCardSticker>,
  /// Address of the NFT of one of the stickers, whose colors must be used for the card background; may be empty if the default background must be used
  background_nft_address: String,
}

impl RObject for TonWalletCard {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletCard"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletCard {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletCardBuilder {
    let inner = TonWalletCard {
      td_name: "tonWalletCard".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletCardBuilder { inner }
  }

  pub fn address(&self) -> &String {
    &self.address
  }

  pub fn gram_amount(&self) -> i64 {
    self.gram_amount
  }

  pub fn stickers(&self) -> &Vec<TonWalletCardSticker> {
    &self.stickers
  }

  pub fn background_nft_address(&self) -> &String {
    &self.background_nft_address
  }
}

#[doc(hidden)]
pub struct RTDTonWalletCardBuilder {
  inner: TonWalletCard,
}

impl RTDTonWalletCardBuilder {
  pub fn build(&self) -> TonWalletCard {
    self.inner.clone()
  }

  pub fn address<T: AsRef<str>>(&mut self, address: T) -> &mut Self {
    self.inner.address = address.as_ref().to_string();
    self
  }

  pub fn gram_amount(&mut self, gram_amount: i64) -> &mut Self {
    self.inner.gram_amount = gram_amount;
    self
  }

  pub fn stickers(&mut self, stickers: Vec<TonWalletCardSticker>) -> &mut Self {
    self.inner.stickers = stickers;
    self
  }

  pub fn background_nft_address<T: AsRef<str>>(&mut self, background_nft_address: T) -> &mut Self {
    self.inner.background_nft_address = background_nft_address.as_ref().to_string();
    self
  }
}

impl AsRef<TonWalletCard> for TonWalletCard {
  fn as_ref(&self) -> &TonWalletCard {
    self
  }
}

impl AsRef<TonWalletCard> for RTDTonWalletCardBuilder {
  fn as_ref(&self) -> &TonWalletCard {
    &self.inner
  }
}
