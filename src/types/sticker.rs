use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a sticker
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Sticker {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique sticker identifier within the set; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Identifier of the sticker set to which the sticker belongs; 0 if none
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  set_id: isize,
  /// Sticker width; as defined by the sender
  width: i64,
  /// Sticker height; as defined by the sender
  height: i64,
  /// Emoji corresponding to the sticker; may be empty if unknown
  emoji: String,
  /// Sticker format
  format: StickerFormat,
  /// Sticker's full type
  full_type: StickerFullType,
  /// Sticker thumbnail in WEBP or JPEG format; may be null
  thumbnail: Option<Thumbnail>,
  /// File containing the sticker
  sticker: File,
}

impl RObject for Sticker {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "sticker"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl Sticker {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStickerBuilder {
    let inner = Sticker {
      td_name: "sticker".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStickerBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn set_id(&self) -> isize {
    self.set_id
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }

  pub fn emoji(&self) -> &String {
    &self.emoji
  }

  pub fn format(&self) -> &StickerFormat {
    &self.format
  }

  pub fn full_type(&self) -> &StickerFullType {
    &self.full_type
  }

  pub fn thumbnail(&self) -> &Option<Thumbnail> {
    &self.thumbnail
  }

  pub fn sticker(&self) -> &File {
    &self.sticker
  }
}

#[doc(hidden)]
pub struct RTDStickerBuilder {
  inner: Sticker,
}

impl RTDStickerBuilder {
  pub fn build(&self) -> Sticker {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn set_id(&mut self, set_id: isize) -> &mut Self {
    self.inner.set_id = set_id;
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

  pub fn emoji<T: AsRef<str>>(&mut self, emoji: T) -> &mut Self {
    self.inner.emoji = emoji.as_ref().to_string();
    self
  }

  pub fn format<T: AsRef<StickerFormat>>(&mut self, format: T) -> &mut Self {
    self.inner.format = format.as_ref().clone();
    self
  }

  pub fn full_type<T: AsRef<StickerFullType>>(&mut self, full_type: T) -> &mut Self {
    self.inner.full_type = full_type.as_ref().clone();
    self
  }

  pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

  pub fn sticker<T: AsRef<File>>(&mut self, sticker: T) -> &mut Self {
    self.inner.sticker = sticker.as_ref().clone();
    self
  }
}

impl AsRef<Sticker> for Sticker {
  fn as_ref(&self) -> &Sticker {
    self
  }
}

impl AsRef<Sticker> for RTDStickerBuilder {
  fn as_ref(&self) -> &Sticker {
    &self.inner
  }
}
