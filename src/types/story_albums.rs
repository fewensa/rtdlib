use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of story albums
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAlbums {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// List of story albums
  albums: Vec<StoryAlbum>,
}

impl RObject for StoryAlbums {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "storyAlbums"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl StoryAlbums {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDStoryAlbumsBuilder {
    let inner = StoryAlbums {
      td_name: "storyAlbums".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDStoryAlbumsBuilder { inner }
  }

  pub fn albums(&self) -> &Vec<StoryAlbum> {
    &self.albums
  }
}

#[doc(hidden)]
pub struct RTDStoryAlbumsBuilder {
  inner: StoryAlbums,
}

impl RTDStoryAlbumsBuilder {
  pub fn build(&self) -> StoryAlbums {
    self.inner.clone()
  }

  pub fn albums(&mut self, albums: Vec<StoryAlbum>) -> &mut Self {
    self.inner.albums = albums;
    self
  }
}

impl AsRef<StoryAlbums> for StoryAlbums {
  fn as_ref(&self) -> &StoryAlbums {
    self
  }
}

impl AsRef<StoryAlbums> for RTDStoryAlbumsBuilder {
  fn as_ref(&self) -> &StoryAlbums {
    &self.inner
  }
}
