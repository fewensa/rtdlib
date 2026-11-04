use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a paid media to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPaidMedia {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Type of the media
  #[serde(rename(serialize = "type", deserialize = "type"))]
  type_: InputPaidMediaType,
  /// Photo or video to be sent
  media: InputFile,
  /// Media thumbnail; pass null to skip thumbnail uploading
  thumbnail: InputThumbnail,
  /// File identifiers of the stickers added to the media, if applicable
  added_sticker_file_ids: Vec<i64>,
  /// Media width
  width: i64,
  /// Media height
  height: i64,
}

impl RObject for InputPaidMedia {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPaidMedia"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputPaidMedia {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPaidMediaBuilder {
    let inner = InputPaidMedia {
      td_name: "inputPaidMedia".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPaidMediaBuilder { inner }
  }

  pub fn type_(&self) -> &InputPaidMediaType {
    &self.type_
  }

  pub fn media(&self) -> &InputFile {
    &self.media
  }

  pub fn thumbnail(&self) -> &InputThumbnail {
    &self.thumbnail
  }

  pub fn added_sticker_file_ids(&self) -> &Vec<i64> {
    &self.added_sticker_file_ids
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }
}

#[doc(hidden)]
pub struct RTDInputPaidMediaBuilder {
  inner: InputPaidMedia,
}

impl RTDInputPaidMediaBuilder {
  pub fn build(&self) -> InputPaidMedia {
    self.inner.clone()
  }

  pub fn type_<T: AsRef<InputPaidMediaType>>(&mut self, type_: T) -> &mut Self {
    self.inner.type_ = type_.as_ref().clone();
    self
  }

  pub fn media<T: AsRef<InputFile>>(&mut self, media: T) -> &mut Self {
    self.inner.media = media.as_ref().clone();
    self
  }

  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

  pub fn added_sticker_file_ids(&mut self, added_sticker_file_ids: Vec<i64>) -> &mut Self {
    self.inner.added_sticker_file_ids = added_sticker_file_ids;
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

impl AsRef<InputPaidMedia> for InputPaidMedia {
  fn as_ref(&self) -> &InputPaidMedia {
    self
  }
}

impl AsRef<InputPaidMedia> for RTDInputPaidMediaBuilder {
  fn as_ref(&self) -> &InputPaidMedia {
    &self.inner
  }
}
