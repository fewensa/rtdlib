use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes gifts available for resale
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GiftsForResale {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of gifts found
  total_count: i64,
  /// The gifts
  gifts: Vec<GiftForResale>,
  /// Available models; for searchGiftsForResale requests without offset and attributes only
  models: Vec<UpgradedGiftModelCount>,
  /// Available symbols; for searchGiftsForResale requests without offset and attributes only
  symbols: Vec<UpgradedGiftSymbolCount>,
  /// Available backdrops; for searchGiftsForResale requests without offset and attributes only
  backdrops: Vec<UpgradedGiftBackdropCount>,
  /// The offset for the next request. If empty, then there are no more results
  next_offset: String,
}

impl RObject for GiftsForResale {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "giftsForResale"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl GiftsForResale {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDGiftsForResaleBuilder {
    let inner = GiftsForResale {
      td_name: "giftsForResale".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDGiftsForResaleBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn gifts(&self) -> &Vec<GiftForResale> {
    &self.gifts
  }

  pub fn models(&self) -> &Vec<UpgradedGiftModelCount> {
    &self.models
  }

  pub fn symbols(&self) -> &Vec<UpgradedGiftSymbolCount> {
    &self.symbols
  }

  pub fn backdrops(&self) -> &Vec<UpgradedGiftBackdropCount> {
    &self.backdrops
  }

  pub fn next_offset(&self) -> &String {
    &self.next_offset
  }
}

#[doc(hidden)]
pub struct RTDGiftsForResaleBuilder {
  inner: GiftsForResale,
}

impl RTDGiftsForResaleBuilder {
  pub fn build(&self) -> GiftsForResale {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn gifts(&mut self, gifts: Vec<GiftForResale>) -> &mut Self {
    self.inner.gifts = gifts;
    self
  }

  pub fn models(&mut self, models: Vec<UpgradedGiftModelCount>) -> &mut Self {
    self.inner.models = models;
    self
  }

  pub fn symbols(&mut self, symbols: Vec<UpgradedGiftSymbolCount>) -> &mut Self {
    self.inner.symbols = symbols;
    self
  }

  pub fn backdrops(&mut self, backdrops: Vec<UpgradedGiftBackdropCount>) -> &mut Self {
    self.inner.backdrops = backdrops;
    self
  }

  pub fn next_offset<T: AsRef<str>>(&mut self, next_offset: T) -> &mut Self {
    self.inner.next_offset = next_offset.as_ref().to_string();
    self
  }
}

impl AsRef<GiftsForResale> for GiftsForResale {
  fn as_ref(&self) -> &GiftsForResale {
    self
  }
}

impl AsRef<GiftsForResale> for RTDGiftsForResaleBuilder {
  fn as_ref(&self) -> &GiftsForResale {
    &self.inner
  }
}
