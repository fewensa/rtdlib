
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Represents a thumbnail
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Thumbnail {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Thumbnail format
  format: ThumbnailFormat,
  /// Thumbnail width
  width: i64,
  /// Thumbnail height
  height: i64,
  /// The thumbnail
  file: File,
  
}

impl RObject for Thumbnail {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "thumbnail" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Thumbnail {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDThumbnailBuilder {
    let mut inner = Thumbnail::default();
    inner.td_name = "thumbnail".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDThumbnailBuilder { inner }
  }

  pub fn format(&self) -> &ThumbnailFormat { &self.format }

  pub fn width(&self) -> i64 { self.width }

  pub fn height(&self) -> i64 { self.height }

  pub fn file(&self) -> &File { &self.file }

}

#[doc(hidden)]
pub struct RTDThumbnailBuilder {
  inner: Thumbnail
}

impl RTDThumbnailBuilder {
  pub fn build(&self) -> Thumbnail { self.inner.clone() }

   
  pub fn format<T: AsRef<ThumbnailFormat>>(&mut self, format: T) -> &mut Self {
    self.inner.format = format.as_ref().clone();
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

   
  pub fn file<T: AsRef<File>>(&mut self, file: T) -> &mut Self {
    self.inner.file = file.as_ref().clone();
    self
  }

}

impl AsRef<Thumbnail> for Thumbnail {
  fn as_ref(&self) -> &Thumbnail { self }
}

impl AsRef<Thumbnail> for RTDThumbnailBuilder {
  fn as_ref(&self) -> &Thumbnail { &self.inner }
}



