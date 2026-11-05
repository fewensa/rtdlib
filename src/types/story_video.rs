use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a video file posted as a story
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Duration of the video, in seconds
  duration: f32,
  /// Video width
  width: i64,
  /// Video height
  height: i64,
  /// True, if stickers were added to the video. The list of corresponding sticker sets can be received using getAttachedStickerSets
  has_stickers: bool,
  /// True, if the video has no sound
  is_animation: bool,
  /// Video minithumbnail; may be null
  minithumbnail: Option<Minithumbnail>,
  /// Video thumbnail in JPEG or MPEG4 format; may be null
  thumbnail: Option<Thumbnail>,
  /// Size of file prefix, which is expected to be preloaded, in bytes
  preload_prefix_size: i64,
  /// Timestamp of the frame used as video thumbnail
  cover_frame_timestamp: f32,
  /// File containing the video
  video: File,
}

impl RObject for StoryVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryVideoBuilder {
    let inner = StoryVideo {
      td_name: "storyVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryVideoBuilder { inner }
  }

  pub fn duration(&self) -> f32 {
    self.duration
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }

  pub fn has_stickers(&self) -> bool {
    self.has_stickers
  }

  pub fn is_animation(&self) -> bool {
    self.is_animation
  }

  pub fn minithumbnail(&self) -> &Option<Minithumbnail> {
    &self.minithumbnail
  }

  pub fn thumbnail(&self) -> &Option<Thumbnail> {
    &self.thumbnail
  }

  pub fn preload_prefix_size(&self) -> i64 {
    self.preload_prefix_size
  }

  pub fn cover_frame_timestamp(&self) -> f32 {
    self.cover_frame_timestamp
  }

  pub fn video(&self) -> &File {
    &self.video
  }
}

#[doc(hidden)]
pub struct RTDStoryVideoBuilder {
  inner: StoryVideo,
}

impl RTDStoryVideoBuilder {
  pub fn build(&self) -> StoryVideo {
    self.inner.clone()
  }

  pub fn duration(&mut self, duration: f32) -> &mut Self {
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

  pub fn has_stickers(&mut self, has_stickers: bool) -> &mut Self {
    self.inner.has_stickers = has_stickers;
    self
  }

  pub fn is_animation(&mut self, is_animation: bool) -> &mut Self {
    self.inner.is_animation = is_animation;
    self
  }

  pub fn minithumbnail<T: AsRef<Minithumbnail>>(&mut self, minithumbnail: T) -> &mut Self {
    self.inner.minithumbnail = Some(minithumbnail.as_ref().clone());
    self
  }

  pub fn thumbnail<T: AsRef<Thumbnail>>(&mut self, thumbnail: T) -> &mut Self {
    self.inner.thumbnail = Some(thumbnail.as_ref().clone());
    self
  }

  pub fn preload_prefix_size(&mut self, preload_prefix_size: i64) -> &mut Self {
    self.inner.preload_prefix_size = preload_prefix_size;
    self
  }

  pub fn cover_frame_timestamp(&mut self, cover_frame_timestamp: f32) -> &mut Self {
    self.inner.cover_frame_timestamp = cover_frame_timestamp;
    self
  }

  pub fn video<T: AsRef<File>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }
}

impl AsRef<StoryVideo> for StoryVideo {
  fn as_ref(&self) -> &StoryVideo {
    self
  }
}

impl AsRef<StoryVideo> for RTDStoryVideoBuilder {
  fn as_ref(&self) -> &StoryVideo {
    &self.inner
  }
}
