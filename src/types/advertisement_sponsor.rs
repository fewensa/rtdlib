use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Information about the sponsor of an advertisement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AdvertisementSponsor {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// URL of the sponsor to be opened when the advertisement is clicked
  url: String,
  /// Photo of the sponsor; may be null if must not be shown
  photo: Option<Photo>,
  /// Additional optional information about the sponsor to be shown along with the advertisement
  info: String,
}

impl RObject for AdvertisementSponsor {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "advertisementSponsor"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl AdvertisementSponsor {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDAdvertisementSponsorBuilder {
    let inner = AdvertisementSponsor {
      td_name: "advertisementSponsor".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDAdvertisementSponsorBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn photo(&self) -> &Option<Photo> {
    &self.photo
  }

  pub fn info(&self) -> &String {
    &self.info
  }
}

#[doc(hidden)]
pub struct RTDAdvertisementSponsorBuilder {
  inner: AdvertisementSponsor,
}

impl RTDAdvertisementSponsorBuilder {
  pub fn build(&self) -> AdvertisementSponsor {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn photo<T: AsRef<Photo>>(&mut self, photo: T) -> &mut Self {
    self.inner.photo = Some(photo.as_ref().clone());
    self
  }

  pub fn info<T: AsRef<str>>(&mut self, info: T) -> &mut Self {
    self.inner.info = info.as_ref().to_string();
    self
  }
}

impl AsRef<AdvertisementSponsor> for AdvertisementSponsor {
  fn as_ref(&self) -> &AdvertisementSponsor {
    self
  }
}

impl AsRef<AdvertisementSponsor> for RTDAdvertisementSponsorBuilder {
  fn as_ref(&self) -> &AdvertisementSponsor {
    &self.inner
  }
}
