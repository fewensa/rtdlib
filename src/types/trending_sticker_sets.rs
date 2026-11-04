use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of trending sticker sets
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrendingStickerSets {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Approximate total number of trending sticker sets
  total_count: i64,
  /// List of trending sticker sets
  sets: Vec<StickerSetInfo>,
  /// True, if the list contains sticker sets with premium stickers
  is_premium: bool,
}

impl RObject for TrendingStickerSets {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "trendingStickerSets"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TrendingStickerSets {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTrendingStickerSetsBuilder {
    let inner = TrendingStickerSets {
      td_name: "trendingStickerSets".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTrendingStickerSetsBuilder { inner }
  }

  pub fn total_count(&self) -> i64 {
    self.total_count
  }

  pub fn sets(&self) -> &Vec<StickerSetInfo> {
    &self.sets
  }

  pub fn is_premium(&self) -> bool {
    self.is_premium
  }
}

#[doc(hidden)]
pub struct RTDTrendingStickerSetsBuilder {
  inner: TrendingStickerSets,
}

impl RTDTrendingStickerSetsBuilder {
  pub fn build(&self) -> TrendingStickerSets {
    self.inner.clone()
  }

  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

  pub fn sets(&mut self, sets: Vec<StickerSetInfo>) -> &mut Self {
    self.inner.sets = sets;
    self
  }

  pub fn is_premium(&mut self, is_premium: bool) -> &mut Self {
    self.inner.is_premium = is_premium;
    self
  }
}

impl AsRef<TrendingStickerSets> for TrendingStickerSets {
  fn as_ref(&self) -> &TrendingStickerSets {
    self
  }
}

impl AsRef<TrendingStickerSets> for RTDTrendingStickerSetsBuilder {
  fn as_ref(&self) -> &TrendingStickerSets {
    &self.inner
  }
}
