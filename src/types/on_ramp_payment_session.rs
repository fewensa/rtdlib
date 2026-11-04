use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a session for an on-ramp purchase
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OnRampPaymentSession {
  #[doc(hidden)]
  #[serde(rename(serialize = "@type", deserialize = "@type"))]
  td_name: String,
  #[doc(hidden)]
  #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
  extra: Option<String>,
  /// Identifier of the session
  session_id: String,
  /// Point in time (Unix timestamp) when the session will expire
  expiration_date: i64,
  /// HTTPS URL to be opened in a WebView to complete the purchase
  url: String,
}

impl RObject for OnRampPaymentSession {
  #[doc(hidden)]
  fn td_name(&self) -> &'static str {
    "onRampPaymentSession"
  }
  #[doc(hidden)]
  fn extra(&self) -> Option<String> {
    self.extra.clone()
  }
  fn to_json(&self) -> RTDResult<String> {
    Ok(serde_json::to_string(self)?)
  }
}

impl OnRampPaymentSession {
  pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
    Ok(serde_json::from_str(json.as_ref())?)
  }
  pub fn builder() -> RTDOnRampPaymentSessionBuilder {
    let inner = OnRampPaymentSession {
      td_name: "onRampPaymentSession".to_string(),
      extra: Some(Uuid::new_v4().to_string()),
      ..Default::default()
    };
    RTDOnRampPaymentSessionBuilder { inner }
  }

  pub fn session_id(&self) -> &String {
    &self.session_id
  }

  pub fn expiration_date(&self) -> i64 {
    self.expiration_date
  }

  pub fn url(&self) -> &String {
    &self.url
  }
}

#[doc(hidden)]
pub struct RTDOnRampPaymentSessionBuilder {
  inner: OnRampPaymentSession,
}

impl RTDOnRampPaymentSessionBuilder {
  pub fn build(&self) -> OnRampPaymentSession {
    self.inner.clone()
  }

  pub fn session_id<T: AsRef<str>>(&mut self, session_id: T) -> &mut Self {
    self.inner.session_id = session_id.as_ref().to_string();
    self
  }

  pub fn expiration_date(&mut self, expiration_date: i64) -> &mut Self {
    self.inner.expiration_date = expiration_date;
    self
  }

  pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
    self.inner.url = url.as_ref().to_string();
    self
  }
}

impl AsRef<OnRampPaymentSession> for OnRampPaymentSession {
  fn as_ref(&self) -> &OnRampPaymentSession {
    self
  }
}

impl AsRef<OnRampPaymentSession> for RTDOnRampPaymentSessionBuilder {
  fn as_ref(&self) -> &OnRampPaymentSession {
    &self.inner
  }
}
