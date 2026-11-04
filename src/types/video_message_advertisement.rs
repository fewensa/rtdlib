use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an advertisement to be shown while a video from a message is watched
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoMessageAdvertisement {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Unique identifier of this result
  unique_id: i64,
  /// Text of the advertisement
  text: String,
  /// The minimum amount of time the advertisement must be displayed before it can be hidden by the user, in seconds
  min_display_duration: i64,
  /// The maximum amount of time the advertisement must be displayed before it must be automatically hidden, in seconds
  max_display_duration: i64,
  /// True, if the advertisement can be reported to Telegram moderators through reportVideoMessageAdvertisement
  can_be_reported: bool,
  /// Information about the sponsor of the advertisement
  sponsor: AdvertisementSponsor,
  /// Title of the sponsored message
  title: String,
  /// If non-empty, additional information about the sponsored message to be shown along with the message
  additional_info: String,
}

impl RObject for VideoMessageAdvertisement {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "videoMessageAdvertisement"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl VideoMessageAdvertisement {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDVideoMessageAdvertisementBuilder {
    let inner = VideoMessageAdvertisement {
      td_name: "videoMessageAdvertisement".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDVideoMessageAdvertisementBuilder { inner }
  }

  pub fn unique_id(&self) -> i64 {
    self.unique_id
  }

  pub fn text(&self) -> &String {
    &self.text
  }

  pub fn min_display_duration(&self) -> i64 {
    self.min_display_duration
  }

  pub fn max_display_duration(&self) -> i64 {
    self.max_display_duration
  }

  pub fn can_be_reported(&self) -> bool {
    self.can_be_reported
  }

  pub fn sponsor(&self) -> &AdvertisementSponsor {
    &self.sponsor
  }

  pub fn title(&self) -> &String {
    &self.title
  }

  pub fn additional_info(&self) -> &String {
    &self.additional_info
  }
}

#[doc(hidden)]
pub struct RTDVideoMessageAdvertisementBuilder {
  inner: VideoMessageAdvertisement,
}

impl RTDVideoMessageAdvertisementBuilder {
  pub fn build(&self) -> VideoMessageAdvertisement {
    self.inner.clone()
  }

  pub fn unique_id(&mut self, unique_id: i64) -> &mut Self {
    self.inner.unique_id = unique_id;
    self
  }

  pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
    self.inner.text = text.as_ref().to_string();
    self
  }

  pub fn min_display_duration(&mut self, min_display_duration: i64) -> &mut Self {
    self.inner.min_display_duration = min_display_duration;
    self
  }

  pub fn max_display_duration(&mut self, max_display_duration: i64) -> &mut Self {
    self.inner.max_display_duration = max_display_duration;
    self
  }

  pub fn can_be_reported(&mut self, can_be_reported: bool) -> &mut Self {
    self.inner.can_be_reported = can_be_reported;
    self
  }

  pub fn sponsor<T: AsRef<AdvertisementSponsor>>(&mut self, sponsor: T) -> &mut Self {
    self.inner.sponsor = sponsor.as_ref().clone();
    self
  }

  pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
    self.inner.title = title.as_ref().to_string();
    self
  }

  pub fn additional_info<T: AsRef<str>>(&mut self, additional_info: T) -> &mut Self {
    self.inner.additional_info = additional_info.as_ref().to_string();
    self
  }
}

impl AsRef<VideoMessageAdvertisement> for VideoMessageAdvertisement {
  fn as_ref(&self) -> &VideoMessageAdvertisement {
    self
  }
}

impl AsRef<VideoMessageAdvertisement> for RTDVideoMessageAdvertisementBuilder {
  fn as_ref(&self) -> &VideoMessageAdvertisement {
    &self.inner
  }
}
