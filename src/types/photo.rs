
use crate::types::*;
use crate::errors::*;




/// Describes a photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Photo {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Photo identifier; 0 for deleted photos
  id: isize,
  /// True, if stickers were added to the photo
  has_stickers: bool,
  /// Available variants of the photo, in different sizes
  sizes: Vec<PhotoSize>,
  
}

impl RObject for Photo {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "photo" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl Photo {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDPhotoBuilder {
    let mut inner = Photo::default();
    inner.td_name = "photo".to_string();
    RTDPhotoBuilder { inner }
  }

  pub fn id(&self) -> isize { self.id }

  pub fn has_stickers(&self) -> bool { self.has_stickers }

  pub fn sizes(&self) -> &Vec<PhotoSize> { &self.sizes }

}

#[doc(hidden)]
pub struct RTDPhotoBuilder {
  inner: Photo
}

impl RTDPhotoBuilder {
  pub fn build(&self) -> Photo { self.inner.clone() }

   
  pub fn id(&mut self, id: isize) -> &mut Self {
    self.inner.id = id;
    self
  }

   
  pub fn has_stickers(&mut self, has_stickers: bool) -> &mut Self {
    self.inner.has_stickers = has_stickers;
    self
  }

   
  pub fn sizes(&mut self, sizes: Vec<PhotoSize>) -> &mut Self {
    self.inner.sizes = sizes;
    self
  }

}

impl AsRef<Photo> for Photo {
  fn as_ref(&self) -> &Photo { self }
}

impl AsRef<Photo> for RTDPhotoBuilder {
  fn as_ref(&self) -> &Photo { &self.inner }
}



