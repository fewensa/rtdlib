use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a sticker on a TON wallet card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonWalletCardSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Position of the sticker on the card
  position: AreaPosition,
  /// The NFT shown as the sticker. If nft.gift is non-null, then it is enough to draw the sticker and there is no need to download the media of the NFT
  nft: TonNft,
}

impl RObject for TonWalletCardSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonWalletCardSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonWalletCardSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonWalletCardStickerBuilder {
    let inner = TonWalletCardSticker {
      td_name: "tonWalletCardSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonWalletCardStickerBuilder { inner }
  }

  pub fn position(&self) -> &AreaPosition {
    &self.position
  }

  pub fn nft(&self) -> &TonNft {
    &self.nft
  }
}

#[doc(hidden)]
pub struct RTDTonWalletCardStickerBuilder {
  inner: TonWalletCardSticker,
}

impl RTDTonWalletCardStickerBuilder {
  pub fn build(&self) -> TonWalletCardSticker {
    self.inner.clone()
  }

  pub fn position<T: AsRef<AreaPosition>>(&mut self, position: T) -> &mut Self {
    self.inner.position = position.as_ref().clone();
    self
  }

  pub fn nft<T: AsRef<TonNft>>(&mut self, nft: T) -> &mut Self {
    self.inner.nft = nft.as_ref().clone();
    self
  }
}

impl AsRef<TonWalletCardSticker> for TonWalletCardSticker {
  fn as_ref(&self) -> &TonWalletCardSticker {
    self
  }
}

impl AsRef<TonWalletCardSticker> for RTDTonWalletCardStickerBuilder {
  fn as_ref(&self) -> &TonWalletCardSticker {
    &self.inner
  }
}
