
use crate::types::*;
use crate::errors::*;




/// Describes a user profile photo
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ProfilePhoto {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  /// Photo identifier; 0 for an empty photo. Can be used to find a photo in a list of userProfilePhotos
  id: String,
  /// A small (160x160) user profile photo
  small: File,
  /// A big (640x640) user profile photo
  big: File,
  
}

impl RObject for ProfilePhoto {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "profilePhoto" }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ProfilePhoto {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDProfilePhotoBuilder {
    let mut inner = ProfilePhoto::default();
    inner.td_name = "profilePhoto".to_string();
    RTDProfilePhotoBuilder { inner }
  }

  pub fn id(&self) -> &String { &self.id }

  pub fn small(&self) -> &File { &self.small }

  pub fn big(&self) -> &File { &self.big }

}

#[doc(hidden)]
pub struct RTDProfilePhotoBuilder {
  inner: ProfilePhoto
}

impl RTDProfilePhotoBuilder {
  pub fn build(&self) -> ProfilePhoto { self.inner.clone() }

   
  pub fn id<T: AsRef<str>>(&mut self, id: T) -> &mut Self {
    self.inner.id = id.as_ref().to_string();
    self
  }

   
  pub fn small<T: AsRef<File>>(&mut self, small: T) -> &mut Self {
    self.inner.small = small.as_ref().clone();
    self
  }

   
  pub fn big<T: AsRef<File>>(&mut self, big: T) -> &mut Self {
    self.inner.big = big.as_ref().clone();
    self
  }

}

impl AsRef<ProfilePhoto> for ProfilePhoto {
  fn as_ref(&self) -> &ProfilePhoto { self }
}

impl AsRef<ProfilePhoto> for RTDProfilePhotoBuilder {
  fn as_ref(&self) -> &ProfilePhoto { &self.inner }
}



