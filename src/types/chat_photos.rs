
use crate::types::*;
use crate::errors::*;
use uuid::Uuid;




/// Contains a list of chat or user profile photos
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatPhotos {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Total number of photos
  total_count: i64,
  /// List of photos
  photos: Vec<ChatPhoto>,
  
}

impl RObject for ChatPhotos {
  #[doc(hidden)] fn td_name(&self) -> &'static str { "chatPhotos" }
  #[doc(hidden)] fn extra(&self) -> Option<String> { self.extra.clone() }
  fn to_json(&self) -> RTDResult<String> { Ok(serde_json::to_string(self)?) }
}



impl ChatPhotos {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> { Ok(serde_json::from_str(json.as_ref())?) }
  pub fn builder() -> RTDChatPhotosBuilder {
    let mut inner = ChatPhotos::default();
    inner.td_name = "chatPhotos".to_string();
    inner.extra = Some(Uuid::new_v4().to_string());
    RTDChatPhotosBuilder { inner }
  }

  pub fn total_count(&self) -> i64 { self.total_count }

  pub fn photos(&self) -> &Vec<ChatPhoto> { &self.photos }

}

#[doc(hidden)]
pub struct RTDChatPhotosBuilder {
  inner: ChatPhotos
}

impl RTDChatPhotosBuilder {
  pub fn build(&self) -> ChatPhotos { self.inner.clone() }

   
  pub fn total_count(&mut self, total_count: i64) -> &mut Self {
    self.inner.total_count = total_count;
    self
  }

   
  pub fn photos(&mut self, photos: Vec<ChatPhoto>) -> &mut Self {
    self.inner.photos = photos;
    self
  }

}

impl AsRef<ChatPhotos> for ChatPhotos {
  fn as_ref(&self) -> &ChatPhotos { self }
}

impl AsRef<ChatPhotos> for RTDChatPhotosBuilder {
  fn as_ref(&self) -> &ChatPhotos { &self.inner }
}



