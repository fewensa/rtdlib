use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// An animation to be sent
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputAnimation {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Animation file to be sent
  animation: InputFile,
  /// Animation thumbnail; pass null to skip thumbnail uploading
  thumbnail: InputThumbnail,
  /// File identifiers of the stickers added to the animation, if applicable
  added_sticker_file_ids: Vec<i64>,
  /// Duration of the animation, in seconds; may be replaced by the server
  duration: i64,
  /// Width of the animation; may be replaced by the server
  width: i64,
  /// Height of the animation; may be replaced by the server
  height: i64,
}

impl RObject for InputAnimation {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "inputAnimation"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl InputAnimation {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDInputAnimationBuilder {
    let inner = InputAnimation {
      td_name: "inputAnimation".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDInputAnimationBuilder { inner }
  }

  pub fn animation(&self) -> &InputFile {
    &self.animation
  }

  pub fn thumbnail(&self) -> &InputThumbnail {
    &self.thumbnail
  }

  pub fn added_sticker_file_ids(&self) -> &Vec<i64> {
    &self.added_sticker_file_ids
  }

  pub fn duration(&self) -> i64 {
    self.duration
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }
}

#[doc(hidden)]
pub struct RTDInputAnimationBuilder {
  inner: InputAnimation,
}

impl RTDInputAnimationBuilder {
  pub fn build(&self) -> InputAnimation {
    self.inner.clone()
  }

  pub fn animation<T: AsRef<InputFile>>(&mut self, animation: T) -> &mut Self {
    self.inner.animation = animation.as_ref().clone();
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

  pub fn duration(&mut self, duration: i64) -> &mut Self {
    self.inner.duration = duration;
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

impl AsRef<InputAnimation> for InputAnimation {
  fn as_ref(&self) -> &InputAnimation {
    self
  }
}

impl AsRef<InputAnimation> for RTDInputAnimationBuilder {
  fn as_ref(&self) -> &InputAnimation {
    &self.inner
  }
}
