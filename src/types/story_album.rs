use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes album of stories
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAlbum {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the album
  id: i64,
  /// Name of the album
  name: String,
  /// Icon of the album; may be null if none
  photo_icon: Option<Photo>,
  /// Video icon of the album; may be null if none
  video_icon: Option<Video>,
}

impl RObject for StoryAlbum {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAlbum"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryAlbum {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAlbumBuilder {
    let inner = StoryAlbum {
      td_name: "storyAlbum".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAlbumBuilder { inner }
  }

  pub fn id(&self) -> i64 {
    self.id
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn photo_icon(&self) -> &Option<Photo> {
    &self.photo_icon
  }

  pub fn video_icon(&self) -> &Option<Video> {
    &self.video_icon
  }
}

#[doc(hidden)]
pub struct RTDStoryAlbumBuilder {
  inner: StoryAlbum,
}

impl RTDStoryAlbumBuilder {
  pub fn build(&self) -> StoryAlbum {
    self.inner.clone()
  }

  pub fn id(&mut self, id: i64) -> &mut Self {
    self.inner.id = id;
    self
  }

  pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
    self.inner.name = name.as_ref().to_string();
    self
  }

  pub fn photo_icon<T: AsRef<Photo>>(&mut self, photo_icon: T) -> &mut Self {
    self.inner.photo_icon = Some(photo_icon.as_ref().clone());
    self
  }

  pub fn video_icon<T: AsRef<Video>>(&mut self, video_icon: T) -> &mut Self {
    self.inner.video_icon = Some(video_icon.as_ref().clone());
    self
  }
}

impl AsRef<StoryAlbum> for StoryAlbum {
  fn as_ref(&self) -> &StoryAlbum {
    self
  }
}

impl AsRef<StoryAlbum> for RTDStoryAlbumBuilder {
  fn as_ref(&self) -> &StoryAlbum {
    &self.inner
  }
}
