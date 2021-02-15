
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a list of all emoji corresponding to a sticker in a sticker set. The list is only for informational purposes, because a sticker is always sent with a fixed emoji from the corresponding Sticker object
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StickerEmojis {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of emojis
  emojis: Vec<String>,
  
}

impl RObject for StickerEmojis {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "stickerEmojis" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl StickerEmojis {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDStickerEmojisBuilder {
    let mut inner = StickerEmojis::default();
    inner.td_name = "stickerEmojis".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDStickerEmojisBuilder { inner }
  }

  pub fn emojis(&self) -> &Vec<String> { &self.emojis }

}

#[doc(hidden)]
pub struct RTDStickerEmojisBuilder {
  inner: StickerEmojis
}

impl RTDStickerEmojisBuilder {
  pub fn build(&self) -> StickerEmojis { self.inner.clone() }

   
  pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
    self.inner.emojis = emojis;
    self
  }

}

impl AsRef<StickerEmojis> for StickerEmojis {
  fn as_ref(&self) -> &StickerEmojis { self }
}

impl AsRef<StickerEmojis> for RTDStickerEmojisBuilder {
  fn as_ref(&self) -> &StickerEmojis { &self.inner }
}



