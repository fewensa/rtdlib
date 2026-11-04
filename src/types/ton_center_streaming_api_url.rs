use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Contains an HTTP URL for TON Center Streaming API
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TonCenterStreamingApiUrl {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// The URL
  url: String,
  /// Time after which the URL expires
  expires_in: i64,
}

impl RObject for TonCenterStreamingApiUrl {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "tonCenterStreamingApiUrl"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl TonCenterStreamingApiUrl {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDTonCenterStreamingApiUrlBuilder {
    let inner = TonCenterStreamingApiUrl {
      td_name: "tonCenterStreamingApiUrl".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDTonCenterStreamingApiUrlBuilder { inner }
  }

  pub fn url(&self) -> &String {
    &self.url
  }

  pub fn expires_in(&self) -> i64 {
    self.expires_in
  }
}

#[doc(hidden)]
pub struct RTDTonCenterStreamingApiUrlBuilder {
  inner: TonCenterStreamingApiUrl,
}

impl RTDTonCenterStreamingApiUrlBuilder {
  pub fn build(&self) -> TonCenterStreamingApiUrl {
    self.inner.clone()
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }

  pub fn expires_in(&mut self, expires_in: i64) -> &mut Self {
    self.inner.expires_in = expires_in;
    self
  }
}

impl AsRef<TonCenterStreamingApiUrl> for TonCenterStreamingApiUrl {
  fn as_ref(&self) -> &TonCenterStreamingApiUrl {
    self
  }
}

impl AsRef<TonCenterStreamingApiUrl> for RTDTonCenterStreamingApiUrlBuilder {
  fn as_ref(&self) -> &TonCenterStreamingApiUrl {
    &self.inner
  }
}
