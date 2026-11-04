use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A sticker to be added to a sticker set
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewSticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// File with the sticker; must fit in a 512x512 square. For WEBP stickers the file must be in WEBP or PNG format, which will be converted to WEBP server-side. See https://core.telegram.org/animated_stickers#technical-requirements for technical requirements
  sticker: InputFile,
  /// Format of the sticker
  format: StickerFormat,
  /// String with 1-20 emoji corresponding to the sticker
  emojis: String,
  /// Position where the mask is placed; pass null if not specified
  mask_position: MaskPosition,
  /// List of up to 20 keywords with total length up to 64 characters, which can be used to find the sticker
  keywords: Vec<String>,
}

impl RObject for NewSticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "newSticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl NewSticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDNewStickerBuilder {
    let inner = NewSticker {
      td_name: "newSticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDNewStickerBuilder { inner }
  }

  pub fn sticker(&self) -> &InputFile {
    &self.sticker
  }

  pub fn format(&self) -> &StickerFormat {
    &self.format
  }

  pub fn emojis(&self) -> &String {
    &self.emojis
  }

  pub fn mask_position(&self) -> &MaskPosition {
    &self.mask_position
  }

  pub fn keywords(&self) -> &Vec<String> {
    &self.keywords
  }
}

#[doc(hidden)]
pub struct RTDNewStickerBuilder {
  inner: NewSticker,
}

impl RTDNewStickerBuilder {
  pub fn build(&self) -> NewSticker {
    self.inner.clone()
  }

  pub fn sticker<T: AsRef<InputFile>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }

  pub fn format<T: AsRef<StickerFormat>>(&mut self, format: T) -> &mut Self {
    self.inner.format = format.as_ref().clone();
    self
  }

  pub fn emojis<T: AsRef<str>>(&mut self, emojis: T) -> &mut Self {
    self.inner.emojis = emojis.as_ref().to_string();
    self
  }

  pub fn mask_position<T: AsRef<MaskPosition>>(&mut self, mask_position: T) -> &mut Self {
    self.inner.mask_position = mask_position.as_ref().clone();
    self
  }

  pub fn keywords(&mut self, keywords: Vec<String>) -> &mut Self {
    self.inner.keywords = keywords;
    self
  }
}

impl AsRef<NewSticker> for NewSticker {
  fn as_ref(&self) -> &NewSticker {
    self
  }
}

impl AsRef<NewSticker> for RTDNewStickerBuilder {
  fn as_ref(&self) -> &NewSticker {
    &self.inner
  }
}
