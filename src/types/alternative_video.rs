use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an alternative re-encoded quality of a video file
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AlternativeVideo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of the alternative video, which is used in the HLS file
  #[serde(deserialize_with = "serde_aux::field_attributes::deserialize_number_from_string")]
  id: isize,
  /// Video width
  width: i64,
  /// Video height
  height: i64,
  /// Codec used for video file encoding, for example, "h264", "h265", "av1", or "av01"
  codec: String,
  /// HLS file describing the video
  hls_file: File,
  /// File containing the video
  video: File,
}

impl RObject for AlternativeVideo {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "alternativeVideo"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AlternativeVideo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAlternativeVideoBuilder {
    let inner = AlternativeVideo {
      td_name: "alternativeVideo".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAlternativeVideoBuilder { inner }
  }

  pub fn id(&self) -> isize {
    self.id
  }

  pub fn width(&self) -> i64 {
    self.width
  }

  pub fn height(&self) -> i64 {
    self.height
  }

  pub fn codec(&self) -> &String {
    &self.codec
  }

  pub fn hls_file(&self) -> &File {
    &self.hls_file
  }

  pub fn video(&self) -> &File {
    &self.video
  }
}

#[doc(hidden)]
pub struct RTDAlternativeVideoBuilder {
  inner: AlternativeVideo,
}

impl RTDAlternativeVideoBuilder {
  pub fn build(&self) -> AlternativeVideo {
    self.inner.clone()
  }

  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
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

  pub fn codec<T: AsRef<str>>(&mut self, codec: T) -> &mut Self {
    self.inner.codec = codec.as_ref().to_string();
    self
  }

  pub fn hls_file<T: AsRef<File>>(&mut self, hls_file: T) -> &mut Self {
    self.inner.hls_file = hls_file.as_ref().clone();
    self
  }

  pub fn video<T: AsRef<File>>(&mut self, video: T) -> &mut Self {
    self.inner.video = video.as_ref().clone();
    self
  }
}

impl AsRef<AlternativeVideo> for AlternativeVideo {
  fn as_ref(&self) -> &AlternativeVideo {
    self
  }
}

impl AsRef<AlternativeVideo> for RTDAlternativeVideoBuilder {
  fn as_ref(&self) -> &AlternativeVideo {
    &self.inner
  }
}
