use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of TON NFTs
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonNfts {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of NFTs
  nfts: Vec<TonNft>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for TonNfts {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonNfts"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonNfts {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonNftsBuilder {
    let inner = TonNfts {
      td_name: "tonNfts".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonNftsBuilder { inner }
  }

  pub fn nfts(&self) -> &Vec<TonNft> {
    &self.nfts
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDTonNftsBuilder {
  inner: TonNfts,
}

impl RTDTonNftsBuilder {
  pub fn build(&self) -> TonNfts {
    self.inner.clone()
  }

  pub fn nfts(&mut self, nfts: Vec<TonNft>) -> &mut Self {
    self.inner.nfts = nfts;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<TonNfts> for TonNfts {
  fn as_ref(&self) -> &TonNfts {
    self
  }
}

impl AsRef<TonNfts> for RTDTonNftsBuilder {
  fn as_ref(&self) -> &TonNfts {
    &self.inner
  }
}
