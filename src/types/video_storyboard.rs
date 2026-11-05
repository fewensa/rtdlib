use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a storyboard for a video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoStoryboard {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// A JPEG file that contains tiled previews of video
  storyboard_file: File,
  /// Width of a tile
  width: i64,
  /// Height of a tile
  height: i64,
  /// File that describes mapping of position in the video to a tile in the JPEG file
  map_file: File,
}

impl RObject for VideoStoryboard {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "videoStoryboard"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl VideoStoryboard {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDVideoStoryboardBuilder {
    let inner = VideoStoryboard {
      td_name: "videoStoryboard".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDVideoStoryboardBuilder { inner }
  }

  pub fn storyboard_file(&self) -> &File {
    &self.storyboard_file
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }

  pub fn map_file(&self) -> &File {
    &self.map_file
  }
}

#[doc(hidden)]
pub struct RTDVideoStoryboardBuilder {
  inner: VideoStoryboard,
}

impl RTDVideoStoryboardBuilder {
  pub fn build(&self) -> VideoStoryboard {
    self.inner.clone()
  }

  pub fn storyboard_file<T: AsRef<File>>(&mut self, storyboard_file: T) -> &mut Self {
    self.inner.storyboard_file = storyboard_file.as_ref().clone();
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

  pub fn map_file<T: AsRef<File>>(&mut self, map_file: T) -> &mut Self {
    self.inner.map_file = map_file.as_ref().clone();
    self
  }
}

impl AsRef<VideoStoryboard> for VideoStoryboard {
  fn as_ref(&self) -> &VideoStoryboard {
    self
  }
}

impl AsRef<VideoStoryboard> for RTDVideoStoryboardBuilder {
  fn as_ref(&self) -> &VideoStoryboard {
    &self.inner
  }
}
