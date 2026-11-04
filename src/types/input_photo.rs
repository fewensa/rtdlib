use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// A photo to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputPhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Photo to be sent. The photo must be at most 10 MB in size. The photo's width and height must not exceed 10000 in total. Width and height ratio must be at most 20
  photo: InputFile,
  /// Photo thumbnail; pass null to skip thumbnail uploading. The thumbnail is sent to the other party only in secret chats
  thumbnail: InputThumbnail,
  /// Video of the live photo; not supported in secret chats; pass null if the photo isn't a live photo
  video: InputFile,
  /// File identifiers of the stickers added to the photo, if applicable
  added_sticker_file_ids: Vec<i64>,
  /// Photo width; may be replaced by the server
  width: i64,
  /// Photo height; may be replaced by the server
  height: i64,
}

impl RObject for InputPhoto {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputPhoto"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputPhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputPhotoBuilder {
    let inner = InputPhoto {
      td_name: "inputPhoto".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputPhotoBuilder { inner }
  }

  pub fn photo(&self) -> &InputFile {
    &self.photo
  }

  pub fn thumbnail(&self) -> &InputThumbnail {
    &self.thumbnail
  }

  pub fn video(&self) -> &InputFile {
    &self.video
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
pub struct RTDInputPhotoBuilder {
  inner: InputPhoto,
}

impl RTDInputPhotoBuilder {
  pub fn build(&self) -> InputPhoto {
    self.inner.clone()
  }

  pub fn photo<T: AsRef<InputFile>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = photo.as_ref().clone();
    self
  }

  pub fn thumbnail<T: AsRef<InputThumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = thumbnail.as_ref().clone();
    self
  }

  pub fn video<T: AsRef<InputFile>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
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

impl AsRef<InputPhoto> for InputPhoto {
  fn as_ref(&self) -> &InputPhoto {
    self
  }
}

impl AsRef<InputPhoto> for RTDInputPhotoBuilder {
  fn as_ref(&self) -> &InputPhoto {
    &self.inner
  }
}
