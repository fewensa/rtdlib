use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A sticker to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Sticker to be sent
  sticker: InputFile,
  /// Sticker thumbnail; pass null to skip thumbnail uploading
  thumbnail: InputThumbnail,
  /// Sticker width
  width: i64,
  /// Sticker height
  height: i64,
}

impl RObject for InputSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputStickerBuilder {
    let inner = InputSticker {
      td_name: "inputSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile {
    &self.sticker
  }

  pub fn thumbnail(&self) -> &InputThumbnail {
    &self.thumbnail
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }
}

#[doc(hidden)]
pub struct RTDInputStickerBuilder {
  inner: InputSticker,
}

impl RTDInputStickerBuilder {
  pub fn build(&self) -> InputSticker {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

  pub fn width(&mut self, width: i64) -> &mut Self {
    self.inner.width = width;
    self
  }

  pub fn height(&mut self, height: i64) -> &mut Self {
    self.inner.height = height;
    self
  }
}

impl AsRef<InputSticker> for InputSticker {
  fn as_ref(&self) -> &InputSticker {
    self
  }
}

impl AsRef<InputSticker> for RTDInputStickerBuilder {
  fn as_ref(&self) -> &InputSticker {
    &self.inner
  }
}
