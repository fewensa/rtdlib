use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a sticker to be placed on the TON wallet card of the current user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputTonWalletCardSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Position of the sticker on the card
  position: AreaPosition,
  /// Address of an NFT owned by the wallet, which is an upgraded gift; use getTonWalletNfts to get the list of owned NFTs
  nft_address: String,
}

impl RObject for InputTonWalletCardSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputTonWalletCardSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputTonWalletCardSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputTonWalletCardStickerBuilder {
    let inner = InputTonWalletCardSticker {
      td_name: "inputTonWalletCardSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputTonWalletCardStickerBuilder { inner }
  }

  pub fn position(&self) -> &AreaPosition {
    &self.position
  }

  pub fn nft_address(&self) -> &String {
    &self.nft_address
  }
}

#[doc(hidden)]
pub struct RTDInputTonWalletCardStickerBuilder {
  inner: InputTonWalletCardSticker,
}

impl RTDInputTonWalletCardStickerBuilder {
  pub fn build(&self) -> InputTonWalletCardSticker {
    self.inner.clone()
  }

  pub fn position<T: AsRef<AreaPosition>>(&mut self, position: T) -> &mut Self {
    self.inner.position = position.as_ref().clone();
    self
  }

  pub fn nft_address<T: AsRef<str>>(&mut self, nft_address: T) -> &mut Self {
    self.inner.nft_address = nft_address.as_ref().to_string();
    self
  }
}

impl AsRef<InputTonWalletCardSticker> for InputTonWalletCardSticker {
  fn as_ref(&self) -> &InputTonWalletCardSticker {
    self
  }
}

impl AsRef<InputTonWalletCardSticker> for RTDInputTonWalletCardStickerBuilder {
  fn as_ref(&self) -> &InputTonWalletCardSticker {
    &self.inner
  }
}
